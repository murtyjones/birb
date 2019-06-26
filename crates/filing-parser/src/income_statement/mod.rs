// standard library / core
use core::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

// html parsing
use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::{SliceExt, TendrilSink};
use html5ever::{LocalName, Namespace, Prefix, QualName};

// regex
use html5ever::tree_builder::Attribute;
use regex::{Regex, RegexBuilder};
use std::ascii::escape_default;

// helpers
use crate::helpers::get_parent_and_index;

lazy_static! {
    static ref INCOME_STATEMENT_HEADER_PATTERN: &'static str = r"
        ^
        (<b>)*                          # Optional closing tag
        (condensed)*\s*                 # The word 'condensed' may be at the beginning (before 'consolidated')
        consolidated\s+                 # 'consolidated '
        (condensed)*\s*                 # The word 'condensed' may be at the beginning (after 'consolidated')
        statements\s+                   # 'statements '
        of\s+                           # 'of '
        (income|operations|earnings)\s* # 'income' or 'operations' or 'earnings', possibly with a space
        (\(loss\))*                     # The word '(loss)' may be at the end
        (and\s+comprehensive\s+loss)*   # The term 'and comprehensive loss' may be at the end
        (and\s+comprehensive\s+income)* # The term 'and comprehensive income' may be at the end
        (</b>)*                         # Optional closing tag
        \s*                             # Optional whitespace
        $
    ";
    static ref INCOME_STATEMENT_HEADER_REGEX: Regex =
        RegexBuilder::new(&INCOME_STATEMENT_HEADER_PATTERN)
            .case_insensitive(true)
            .multi_line(true)
            .ignore_whitespace(true)
            .build()
            .expect("Couldn't build income statement regex!");
}

const MAX_LEVELS_UP: i32 = 1;
// TODO: In the actual rendering of a document, this looks like it should only be a few levels over.
// However when html5ever parses it into a dom, 8 levels over is required. Could just be because of text nodes,
// but it's worth ensuring that there isn't whitespace or something being converted to a node unneccesarily.
const MAX_LEVELS_OVER: i32 = 8;

pub struct DomifiedFiling {
    pub dom: RcDom,
    pub income_statement_table_found: bool,
    pub path_to_income_statement_header_node: Option<Vec<i32>>,
    pub viable_income_statement_header_node: Option<Handle>,
    pub viable_header_node_has_table_nearby: Option<bool>,
}

#[allow(dead_code)]
impl DomifiedFiling {
    fn new(filing_contents: String) -> DomifiedFiling {
        DomifiedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            income_statement_table_found: false,
            path_to_income_statement_header_node: None,
            viable_income_statement_header_node: None,
            viable_header_node_has_table_nearby: None,
        }
    }

    fn get_doc(&self) -> Rc<Node> {
        Rc::clone(&self.dom.document)
    }

    fn has_income_statement_header_text(&self, text: &str) -> bool {
        INCOME_STATEMENT_HEADER_REGEX.is_match(text)
    }

    fn start_walker(&mut self) {
        let doc = self.get_doc();
        let path_to_node = vec![];
        self.walker(&doc, path_to_node);
    }

    fn current_node_is_income_statement_header(
        &mut self,
        handle: &Handle,
        node_contents: String,
        parent: &Handle,
        child_index: i32,
    ) -> bool {
        // If the node text doesn't match the income statement RegExp, return false
        if !self.has_income_statement_header_text(&node_contents) {
            return false;
        }
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
                if self.offset_node_is_a_table_element(parent, child_index, sibling_index) {
                    return true;
                }
            }
        }

        // If we didn't find a nearby table, return false
        false
    }

    fn offset_node_is_a_table_element(
        &mut self,
        parent: &Handle,
        child_index: i32,
        sibling_offset: i32,
    ) -> bool {
        let sibling_index_from_parent = child_index + sibling_offset;
        let children = &parent.children.borrow();
        // There may not be a sibling at the offset specified, in which case
        // we return "false"
        if (children.len() as i32 - 1) < sibling_index_from_parent as i32 {
            return false;
        }
        let mut sibling = &children[sibling_index_from_parent as usize];
        self.node_is_table_element(sibling)
    }

    fn node_is_table_element(&mut self, node: &Handle) -> bool {
        match node.data {
            NodeData::Element { ref name, .. } => {
                println!("<{}>", &name.local);
                &name.local == "table"
            }
            _ => false,
        }
    }

    fn walker(&mut self, handle: &Handle, path_to_node: Vec<i32>) {
        let node = handle;

        // If the income statement was already found, stop walking the DOM
        if self.income_statement_table_found {
            return ();
        }

        match node.data {
            NodeData::Text { ref contents } => {
                let (parent, child_index) =
                    get_parent_and_index(handle).expect("Couldn't get parent node and index.");
                let mut node_contents = String::new();
                node_contents.push_str(&contents.borrow());
                if self.current_node_is_income_statement_header(
                    handle,
                    node_contents,
                    &parent,
                    child_index,
                ) {
                    self.borrow_mut().path_to_income_statement_header_node =
                        Some(path_to_node.clone());
                    self.borrow_mut().income_statement_table_found == true;
                    match parent.data {
                        NodeData::Element { ref attrs, .. } => {
                            self.attach_style_to_header(attrs);
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
            let mut path_to_child_node = path_to_node.clone();
            path_to_child_node.push(i as i32);
            &self.walker(child, path_to_child_node);
        }
    }

    fn attach_style_to_header(&mut self, attrs: &RefCell<Vec<Attribute>>) {
        /*
         * TODO: Once it's verified that the income statement parser works
         * correctly, remove the red background styling stuff below
         * and use a custom id e.g. "x-birb-income-statement-header"
         */

        // Remove the style attribute if it exists
        attrs
            .borrow_mut()
            .retain(|attr| &attr.name.local != "style");

        // Add the custom style attribute
        let colorizer: Attribute = Attribute {
            name: QualName::new(None, ns!(), local_name!("style")),
            value: "background-color: red;".to_tendril(),
        };
        attrs.borrow_mut().push(colorizer);
    }

    fn set_viable_income_statement_header_node(&mut self) {
        match &self.path_to_income_statement_header_node {
            Some(path) => {
                let doc = self.get_doc();
                self.get_node_location(&doc, path.to_owned().borrow_mut());
            }
            None => panic!("Can't get income statement node if none was found!"),
        }
    }

    fn get_node_location(&mut self, handle: &Handle, path: &[i32]) {
        if path.len() == 0 {
            self.borrow_mut().viable_income_statement_header_node = Some(handle.clone());
            ()
        } else {
            let i = &path[0];
            let child = &handle.children.borrow()[*i as usize];
            self.get_node_location(child, &path[1..]);
        }
    }

    #[cfg(test)]
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
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    struct TestableFiling {
        path: String,
        header_inner_html: String,
    }

    lazy_static! {
                static ref FILES: Vec<TestableFiling> = vec![
                    TestableFiling {
                        path: String::from("./examples/10-Q/input/0001193125-18-037381.txt"),
                        header_inner_html: String::from("Consolidated Statements of Income (Loss) "),
                    },
        //            TestableFiling {
        //                path: String::from("./examples/10-Q/input/0001000623-17-000125.txt"),
        //                header_inner_html: String::from("CONDENSED CONSOLIDATED STATEMENTS OF INCOME"),
        //            },
        //            TestableFiling {
        //                path: String::from("./examples/10-Q/input/0001437749-16-025027.txt"),
        //                header_inner_html: String::from(
        //                    "CONDENSED CONSOLIDATED STATEMENTS OF OPERATIONS AND COMPREHENSIVE LOSS"
        //                ),
        //            },
        //            TestableFiling {
        //                path: String::from("./examples/10-Q/input/0001004434-17-000011.txt"),
        //                header_inner_html: String::from("CONSOLIDATED STATEMENTS OF INCOME"),
        //            },
        //            TestableFiling {
        //                path: String::from("./examples/10-Q/input/0001185185-16-005721.txt"),
        //                header_inner_html: String::from("CONSOLIDATED STATEMENTS OF OPERATIONS"),
        //            },
        //            TestableFiling {
        //                path: String::from("./examples/10-Q/input/0001437749-16-036870.txt"),
        //                header_inner_html: String::from(
        //                    "CONSOLIDATED STATEMENTS OF INCOME AND COMPREHENSIVE INCOME"
        //                ),
        //            },
        //            TestableFiling {
        //                path: String::from("./examples/10-Q/input/0001193125-16-454777.txt"),
        //                header_inner_html: String::from("Consolidated Statements of Income "),
        //            },
        //            TestableFiling {
        //                path: String::from("./examples/10-Q/input/0001193125-17-160261.txt"),
        //                header_inner_html: String::from("CONSOLIDATED STATEMENTS OF OPERATIONS "),
        //            },
    //                TestableFiling {
    //                    path: String::from("./examples/10-Q/input/0001001288-16-000069.txt"),
    //                    header_inner_html: String::from("Consolidated Condensed Statements of Earnings "),
    //                },
                ];
            }

    fn get_file_contents(path: &String) -> String {
        let path = Path::new(path.as_str());
        let mut file = File::open(path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't get file contents");
        contents
    }

    fn make_struct(path: &String) -> DomifiedFiling {
        let filing_contents = get_file_contents(path);
        // To parse a string into a tree of nodes, we need to invoke
        // `parse_document` and supply it with a TreeSink implementation (RcDom).
        let domified_filing = DomifiedFiling::new(filing_contents);
        domified_filing
    }

    #[test]
    fn test_income_statement_known_header_examples() {
        for i in 0..FILES.len() {
            let file = &FILES[i];
            assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(&file.header_inner_html));
        }
    }

    #[test]
    fn test_income_statement_header_location_is_found() {
        for i in 0..FILES.len() {
            let file = &FILES[i];
            let mut domified_filing = make_struct(&file.path);
            domified_filing.start_walker();
            assert_ne!(
                domified_filing.path_to_income_statement_header_node, None,
                "There should be at least one income statement header in every document!"
            );
        }
    }

    #[test]
    fn test_income_statement_header_location_is_correct() {
        for i in 0..FILES.len() {
            // Arrange
            let file = &FILES[i];
            let mut domified_filing = make_struct(&file.path);
            let output_path = String::from(format!("./examples/10-Q/output/{}.html", i));

            // Act
            domified_filing.start_walker();
            domified_filing.set_viable_income_statement_header_node();
            domified_filing.write_file_contents(output_path);

            // Assert
            let node = domified_filing.viable_income_statement_header_node.unwrap();
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
