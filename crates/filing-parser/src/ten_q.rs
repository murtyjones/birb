// standard library / core
use core::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

// html parsing
use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::{SliceExt, StrTendril, TendrilSink};
use html5ever::tree_builder::Attribute;
use html5ever::{LocalName, Namespace, Prefix, QualName};

// regex
use crate::regexes::income_statement::INCOME_STATEMENT_HEADER_REGEX;
use std::ascii::escape_default;

// helpers
use crate::helpers::{get_parent_and_index, tendril_to_string};

// test files
use crate::test_files::FILES;

const MAX_LEVELS_UP: i32 = 4;
// TODO: In the actual rendering of a document, this looks like it should only be a few levels over.
// However when html5ever parses it into a dom, 8 levels over is required. Could just be because of text nodes,
// but it's worth ensuring that there isn't whitespace or something being converted to a node unneccesarily.
const MAX_LEVELS_OVER: i32 = 10;

pub struct ProcessedFiling {
    pub dom: RcDom,
    pub income_statement_table_node: Option<Handle>,
    pub income_statement_header_node: Option<Handle>,
}

#[allow(dead_code)]
impl ProcessedFiling {
    fn new(filing_contents: String) -> ProcessedFiling {
        let mut p_f = ProcessedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            income_statement_table_node: None,
            income_statement_header_node: None,
        };

        // Process the filing
        p_f.process();

        // Return the processed document
        p_f
    }

    /// Gets the Node containing the entire parsed document
    fn get_doc(&self) -> Rc<Node> {
        Rc::clone(&self.dom.document)
    }

    /// Does it all!
    fn process(&mut self) {
        let doc = self.get_doc();
        self.maybe_find_income_statement_table(&doc);
        // TODO add other processing steps here
    }

    fn has_income_statement_regex(&self, handle: &Handle) -> bool {
        match handle.data {
            NodeData::Text { ref contents } => {
                let contents = tendril_to_string(contents);
                // If the node text doesn't match the income statement RegExp, exit
                if !INCOME_STATEMENT_HEADER_REGEX.is_match(contents.as_str()) {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
        true
    }

    fn _maybe_find_income_statement_table(
        &mut self,
        handle: &Handle,
        parent: &Handle,
        child_index: i32,
    ) {
        if !self.has_income_statement_regex(handle) {
            return ();
        };

        let mut parents_and_indexes: Vec<(Rc<Node>, i32)> = vec![(Rc::clone(&parent), child_index)];

        // get parents several levels up:
        for i in 1..=MAX_LEVELS_UP {
            let prev_node_index = (i as usize) - 1;
            let prev_node = &parents_and_indexes[prev_node_index].0;
            parents_and_indexes.push(
                get_parent_and_index(prev_node).expect("Couldn't get parent node and index."),
            );
        }

        // for each parent, check if a sibling near to the current child is a table element.
        // if any are, return true.
        for each in parents_and_indexes {
            let parent = &each.0;
            let child_index = each.1;
            for sibling_index in 1..=MAX_LEVELS_OVER {
                if self.income_statement_table_node.is_none() {
                    self.offset_node_is_a_table_element(parent, child_index, sibling_index);
                }
            }
        }
    }

    fn offset_node_is_a_table_element(
        &mut self,
        parent: &Handle,
        child_index: i32,
        sibling_offset: i32,
    ) {
        let sibling_index_from_parent = child_index + sibling_offset;
        let children = &parent.children.borrow();
        // There may not be a sibling at the offset specified, in which case
        // we return "false"
        if (children.len() as i32 - 1) < sibling_index_from_parent as i32 {
            return ();
        }
        let mut sibling = &children[sibling_index_from_parent as usize];
        self.node_is_table_element(sibling);
    }

    fn node_is_table_element(&mut self, node: &Handle) {
        if self.income_statement_table_node.is_some() {
            return ();
        }

        match node.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                println!("<{}>", &name.local);
                if &name.local == "table" {
                    self.borrow_mut().income_statement_table_node = Some(Rc::clone(node));
                    self.add_red_bg_style(attrs);
                    return ();
                }
            }
            _ => {}
        }

        /// Iterate through text and node children looking for
        /// a table element
        for (i, child) in
            node.children
                .borrow()
                .iter()
                .enumerate()
                .filter(|(_i, child)| match child.data {
                    NodeData::Element { .. } => true,
                    _ => false,
                })
        {
            self.node_is_table_element(child);
        }
    }

    fn maybe_find_income_statement_table(&mut self, handle: &Handle) {
        let node = handle;

        // If the income statement was already found, exit
        if self.income_statement_table_node.is_some() {
            return ();
        }

        match node.data {
            NodeData::Text { ref contents } => {
                let (parent, child_index) =
                    get_parent_and_index(handle).expect("Couldn't get parent node and index.");

                // try to find the nearby income statement table
                self._maybe_find_income_statement_table(handle, &parent, child_index);

                // If a table element was found near to the header, denoate this header as
                // the table's header and attach some custom styling to it
                if self.income_statement_table_node.is_some() {
                    self.borrow_mut().income_statement_header_node = Some(handle.clone());
                    match parent.data {
                        NodeData::Element { ref attrs, .. } => {
                            self.add_red_bg_style(attrs);
                        }
                        _ => panic!("Parent should be an element!"),
                    }
                    ()
                }
            }
            _ => {}
        }

        for (i, child) in
            node.children
                .borrow()
                .iter()
                .enumerate()
                .filter(|(_i, child)| match child.data {
                    NodeData::Text { .. } | NodeData::Element { .. } => true,
                    _ => false,
                })
        {
            &self.maybe_find_income_statement_table(child);
        }
    }

    /// Temporary method that attaches styling for visual confrimation.
    /// eventually this should attach a custom data ID.
    /// Something like: x-birb-income-statement-header
    fn add_red_bg_style(&mut self, attrs: &RefCell<Vec<Attribute>>) {
        // Remove the style attribute if it exists (TODO remove this):
        attrs
            .borrow_mut()
            .retain(|attr| &attr.name.local != "style");

        // Add the custom style attribute (TODO make this add a custom ID instead):
        let colorizer: Attribute = Attribute {
            name: QualName::new(None, ns!(), local_name!("style")),
            value: "background-color: red;".to_tendril(),
        };

        // Add the new attributes to the element:
        attrs.borrow_mut().push(colorizer);
    }
}

#[cfg(test)]
impl ProcessedFiling {
    fn write_file_contents(&mut self, path: String) {
        let doc: &Rc<Node> = &self.get_doc();
        let buffer = std::fs::File::create(path).expect("Could't create file.");
        html5ever::serialize::serialize(
            buffer,
            doc,
            html5ever::serialize::SerializeOpts::default(),
        )
        .expect("Couldn't write to file.");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::get_abs_path;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn get_file_contents(path: &String) -> String {
        let path = get_abs_path(path);
        let mut file = File::open(path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't get file contents");
        contents
    }

    fn make_processed_filing(path: &String) -> ProcessedFiling {
        let filing_contents = get_file_contents(path);
        // To parse a string into a tree of nodes, we need to invoke
        // `parse_document` and supply it with a TreeSink implementation (RcDom).
        let processed_filing = ProcessedFiling::new(filing_contents);
        processed_filing
    }

    #[test]
    fn test_income_statement_header_and_table_location_found() {
        for i in 0..FILES.len() {
            let file = &FILES[i];
            let mut processed_filing = make_processed_filing(&file.path);
            assert!(
                processed_filing.income_statement_table_node.is_some(),
                "There should be a table for each income statement!"
            );
            assert!(
                processed_filing.income_statement_header_node.is_some(),
                "There should be a header node for each income statement!"
            );
        }
    }

    #[test]
    fn test_income_statement_header_location_is_correct() {
        for i in 0..FILES.len() {
            // Arrange
            let file = &FILES[i];
            let mut processed_filing = make_processed_filing(&file.path);
            let output_path = String::from(format!("./examples/10-Q/output/{}.html", i));

            // Act
            processed_filing.write_file_contents(output_path);

            // Assert
            let node = processed_filing.income_statement_header_node.unwrap();
            match node.data {
                NodeData::Text { ref contents } => {
                    let mut stringified_contents = String::new();
                    stringified_contents.push_str(&contents.borrow());
                    assert_eq!(file.header_inner_html, stringified_contents);
                }
                _ => panic!("Wrong node!"),
            }
        }
    }
}
