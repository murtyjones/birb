extern crate regex;
extern crate xml5ever;
#[macro_use]
extern crate lazy_static;

// Std requires
use std::ascii::escape_default;
use std::default::Default;
use std::io;
use std::rc::*;
use std::string::String;

// xml requires
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::tendril::TendrilSink;
use xml5ever::tree_builder::TreeSink;

// regex requires
use core::borrow::BorrowMut;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"consolidated\s+statements\s+of\s+(income|operations)").unwrap();
}

pub struct DomifiedFiling {
    pub dom: RcDom,
    pub income_statement_header_node_count: i32,
}

impl DomifiedFiling {
    fn new(filing_contents: String) -> DomifiedFiling {
        DomifiedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            income_statement_header_node_count: 0,
        }
    }

    fn get_doc(&self) -> &Rc<Node> {
        &self.dom.document
    }

    fn is_income_statement_header_node(text: &str) -> bool {
        RE.is_match(text)
    }

    fn walker(&mut self, handle: &Handle) {
        let node = handle;

        match node.data {
            NodeData::Text { ref contents } => {
                let text = contents.borrow();
                if RE.is_match(&text) {
                    self.borrow_mut().income_statement_header_node_count += 1;
                }
            }
            _ => {}
        }

        for child in node
            .children
            .borrow()
            .iter()
            .filter(|child| match child.data {
                NodeData::Text { .. } | NodeData::Element { .. } => true,
                _ => false,
            })
        {
            &self.walker(child);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    const STATIC_10_Q_FILING_NAMES: [&'static str; 1] = ["./src/0001193125-18-037381.txt"];

    fn get_file_contents(filing_name: &'static str) -> String {
        let path = Path::new(filing_name);
        let mut file = File::open(path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't get file contents");
        contents
    }

    fn run_for_one(filing_name: &'static str) -> DomifiedFiling {
        let filing_contents = get_file_contents(filing_name);
        // To parse a string into a tree of nodes, we need to invoke
        // `parse_document` and supply it with a TreeSink implementation (RcDom).
        let mut filing = DomifiedFiling::new(filing_contents);
        let doc = &Rc::clone(filing.get_doc());
        filing.walker(doc);
        filing
    }

    #[test]
    fn test_income_statement_count_greater_than_0() {
        for filer in &STATIC_10_Q_FILING_NAMES {
            let filing = run_for_one(filer);
            assert!(
                filing.income_statement_header_node_count > 0,
                "There should be at least one income statement header in every document!"
            );
        }
    }

    #[test]
    fn test_income_statement_count_less_than_2() {
        for filer in &STATIC_10_Q_FILING_NAMES {
            let filing = run_for_one(filer);
            assert!(
                filing.income_statement_header_node_count < 2,
                format!("There should only be one income statement header in every document! Relevant doc: {}", filer)
            );
        }
    }
}
