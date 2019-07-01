// standard library / core
use core::borrow::{Borrow, BorrowMut};
use failure::Error;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

// html parsing
use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::{SliceExt, StrTendril, TendrilSink};
use html5ever::tree_builder::Attribute;
use html5ever::{LocalName, Namespace, Prefix, QualName};

// regex / text matching
use crate::matching_attributes::MATCHING_ATTRIBUTES;
use crate::regexes::income_statement::INCOME_STATEMENT_HEADER_REGEX;
use std::ascii::escape_default;

// helpers
use crate::helpers::{
    add_attribute, create_x_birb_attr, get_parents_and_indexes, tendril_to_string,
};

// test files
use crate::test_files::FILES;

pub const MAX_LEVELS_UP: i32 = 4;
// TODO: In the actual rendering of a document, this looks like it should only be a few levels over.
// However when html5ever parses it into a dom, 8 levels over is required. Could just be because of text nodes,
// but it's worth ensuring that there isn't whitespace or something being converted to a node unneccesarily.
pub const MAX_LEVELS_OVER: i32 = 10;

pub struct ProcessedFiling {
    pub dom: RcDom,
    pub income_statement_table_node: Option<Handle>,
    pub income_statement_header_node: Option<Handle>,
}

pub enum ProcessingStep {
    IncomeStatement,
}

#[derive(Debug, Fail, PartialEq)]
pub enum ProcessingError {
    #[fail(display = "No income statement found for CIK: {}", cik)]
    NoIncomeStatementFound { cik: String },
}

#[allow(dead_code)]
impl ProcessedFiling {
    pub fn new(filing_contents: String) -> Result<ProcessedFiling, ProcessingError> {
        let mut p_f = ProcessedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            income_statement_table_node: None,
            income_statement_header_node: None,
        };

        // Process the filing
        let result = p_f.process();
        if let Err(e) = result {
            return Err(e);
        }

        // Return the processed document
        Ok(p_f)
    }

    /// Gets the Node containing the entire parsed document
    fn get_doc(&self) -> Rc<Node> {
        Rc::clone(&self.dom.document)
    }

    /// Does it all!
    fn process(&mut self) -> Result<(), ProcessingError> {
        let doc = self.get_doc();

        // Find the income statement
        self.process_step(&doc, &ProcessingStep::IncomeStatement);
        if self.income_statement_header_node.is_none() {
            return Err(ProcessingError::NoIncomeStatementFound {
                cik: String::from("fake"),
            });
        }
        // TODO add other processing steps here
        Ok(())
    }

    fn process_step(&mut self, handle: &Handle, s: &ProcessingStep) {
        match s {
            ProcessingStep::IncomeStatement => {
                self.maybe_find_income_statement_table(handle);
            }
        }
    }

    fn maybe_find_income_statement_table(&mut self, handle: &Handle) {
        let node = handle;
        // If the income statement was already found, exit
        if self.income_statement_table_node.is_some() {
            return ();
        }
        // try to find the nearby income statement table
        self.process_income_statement_if_matching_node_type(handle);
        // If header + table was found, exit
        if self.income_statement_table_node.is_some() {
            return ();
        }
        self.next_iteration(node, &ProcessingStep::IncomeStatement);
    }

    fn next_iteration(&mut self, handle: &Handle, s: &ProcessingStep) {
        for (i, child) in handle
            .children
            .borrow()
            .iter()
            .enumerate()
            .filter(|(_i, child)| match child.data {
                NodeData::Text { .. } | NodeData::Element { .. } => true,
                _ => false,
            })
        {
            &self.process_step(child, &s);
        }
    }

    fn process_income_statement_if_matching_node_type(&mut self, handle: &Handle) {
        match handle.data {
            NodeData::Text { .. } | NodeData::Element { .. } => {
                self.analyze_node_as_possible_income_statement(handle);
            }
            _ => {}
        }
    }

    fn analyze_node_as_possible_income_statement(&mut self, handle: &Handle) {
        if !self.hueristical_income_statement_content_match(handle) {
            return ();
        };

        let parents_and_indexes = get_parents_and_indexes(handle);

        // for each parent, check if a sibling near to the current child is a table element.
        // if any are, return true.
        for each in &parents_and_indexes {
            let parent = &Rc::clone(&each.0);
            let child_index = each.1.clone();
            for sibling_index in 1..=MAX_LEVELS_OVER {
                if self.income_statement_table_node.is_none() {
                    self.offset_node_is_a_table_element(parent, child_index, sibling_index);
                }
            }
        }

        self.maybe_attach_income_statement_attributes(handle, parents_and_indexes);
    }

    fn hueristical_income_statement_content_match(&self, handle: &Handle) -> bool {
        // if a text node with matching regex, return true
        if let NodeData::Text { ref contents } = handle.data {
            let contents = tendril_to_string(contents);
            if INCOME_STATEMENT_HEADER_REGEX.is_match(contents.as_str()) {
                return true;
            }
        }

        // if an element node with matching attributes, return true
        if let NodeData::Element { ref attrs, .. } = handle.data {
            let length = attrs.borrow().len();
            for i in 0..length {
                let attr: &Attribute = &attrs.borrow()[i];
                for j in 0..MATCHING_ATTRIBUTES.len() {
                    let value = &RefCell::new(attr.value.clone());
                    let matching_attr = &MATCHING_ATTRIBUTES[j];
                    let name_matches = &attr.name.local == matching_attr.name;
                    print!("{}=\"{}\"\n", &attr.name.local, tendril_to_string(value));
                    let value_matches = tendril_to_string(value) == matching_attr.value;
                    if name_matches && value_matches {
                        return true;
                    }
                }
            }
        }
        return false;
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
        self.recursive_node_is_table_element(sibling);
    }

    fn recursive_node_is_table_element(&mut self, node: &Handle) {
        if self.income_statement_table_node.is_some() {
            return ();
        }

        if let NodeData::Element {
            ref name,
            ref attrs,
            ..
        } = node.data
        {
            if &name.local == "table" {
                self.borrow_mut().income_statement_table_node = Some(Rc::clone(node));
                return ();
            }
        }

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
            self.recursive_node_is_table_element(child);
        }
    }

    fn maybe_attach_income_statement_attributes(
        &mut self,
        handle: &Handle,
        parents_and_indexes: Vec<(Rc<Node>, i32)>,
    ) {
        if self.income_statement_table_node.is_some() {
            self.borrow_mut().income_statement_header_node = Some(handle.clone());
            // If table was found, attach TEMPORARY red background to immediate parent
            // Add the custom style attribute (TODO remove this eventually):
            let colorizer: Attribute = Attribute {
                name: QualName::new(None, ns!(), local_name!("style")),
                value: "background-color: red;".to_tendril(),
            };
            let immediate_parent = &parents_and_indexes[0].0;
            let table_node = &self.income_statement_table_node.as_ref().unwrap();
            add_attribute(immediate_parent, colorizer.clone(), Some("style"));
            add_attribute(table_node, colorizer.clone(), Some("style"));

            // add custom birb income statement identifier
            add_attribute(
                table_node,
                create_x_birb_attr("x-birb-income-statement-table"),
                None,
            );
        }
    }
}

impl ProcessedFiling {
    pub fn write_file_contents(&mut self, path: &String) {
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
    use crate::test_files::MatchType;
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
        match processed_filing {
            Ok(p_f) => p_f,
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_should_err_when_no_income_statement_found() {
        let fake_html = String::from("<html></html>");
        let processed_filing = ProcessedFiling::new(fake_html);
        assert!(processed_filing.is_err());
        if let Err(e) = processed_filing {
            assert_eq!(
                e,
                ProcessingError::NoIncomeStatementFound {
                    cik: String::from("fake")
                }
            );
        }
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
    fn test_income_statement_header_regex_is_correct() {
        for i in 0..FILES.len() {
            // Arrange
            let file = &FILES[i];
            if file.match_type == MatchType::Regex {
                let mut processed_filing = make_processed_filing(&file.path);
                let output_path = String::from(format!("./examples/10-Q/output/{}.html", i));

                // Act
                processed_filing.write_file_contents(&output_path);

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
}
