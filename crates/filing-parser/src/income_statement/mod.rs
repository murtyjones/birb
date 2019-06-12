// standard library / core
use core::borrow::BorrowMut;
use std::rc::Rc;

// xml
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::tendril::TendrilSink;

// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref INCOME_STATEMENT_HEADER_PATTERN: &'static str = r"
        ^
        (<b>)*                          # Optional closing tag
        (condensed)*\s*                 # The word 'condensed' may be at the beginning
        consolidated\s+                 # 'consolidated '
        statements\s+                   # 'statements '
        of\s+                           # 'of '
        (income|operations)\s*          # 'income' or 'operations', possibly with a space
        (\(loss\))*                     # The word '(loss)' may be at the end
        (and\s+comprehensive\s+loss)*   # The term '(and comprehensive loss)' may be at the end
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

pub struct DomifiedFiling {
    pub dom: RcDom,
    pub path_to_income_statement_node: Option<Vec<i32>>,
}

impl DomifiedFiling {
    fn new(filing_contents: String) -> DomifiedFiling {
        DomifiedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            path_to_income_statement_node: None,
        }
    }

    fn get_doc(&self) -> Rc<Node> {
        Rc::clone(&self.dom.document)
    }

    fn is_income_statement_header_node(&self, text: &str) -> bool {
        INCOME_STATEMENT_HEADER_REGEX.is_match(text)
    }

    fn start_walker(&mut self) {
        let doc = self.get_doc();
        let path_to_node = vec![];
        self.walker(&doc, path_to_node);
    }

    fn walker(&mut self, handle: &Handle, path_to_node: Vec<i32>) {
        let node = handle;

        match node.data {
            NodeData::Text { ref contents } => {
                let text = contents.borrow();
                if self.is_income_statement_header_node(&text) {
                    self.borrow_mut().path_to_income_statement_node = Some(path_to_node.clone());
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

    fn get_income_statement_node(&self) {
        match &self.path_to_income_statement_node {
            Some(path) => {
                let doc = self.get_doc();
                self.go_to_node(&doc, path);
            }
            None => panic!("Can't get income statement node if none was found!"),
        }
    }

    fn go_to_node(&self, handle: &Handle, path: &Vec<i32>) {
        let mut node = handle;
        for i in path.iter() {
            let child = &node.children.borrow()[*i as usize];
            node = child;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    // TODO use S3 when running these in CI.
    const STATIC_10_Q_FILING_NAMES: [&'static str; 4] = [
        "./examples/0001193125-18-037381.txt",
        "./examples/0001000623-17-000125.txt",
        "./examples/0001437749-16-025027.txt",
        "./examples/0001004434-17-000011.txt",
    ];

    const INCOME_STMT_HEADER_INNER_HTML: [&'static str; 4] = [
        "Consolidated Statements of Income (Loss) ",
        "CONDENSED CONSOLIDATED STATEMENTS OF INCOME",
        "<b>CONDENSED CONSOLIDATED STATEMENTS OF OPERATIONS AND COMPREHENSIVE LOSS</b>",
        "CONSOLIDATED STATEMENTS OF INCOME",
    ];

    fn get_file_contents(filing_name: &'static str) -> String {
        let path = Path::new(filing_name);
        let mut file = File::open(path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't get file contents");
        contents
    }

    fn make_struct(filing_name: &'static str) -> DomifiedFiling {
        let filing_contents = get_file_contents(filing_name);
        // To parse a string into a tree of nodes, we need to invoke
        // `parse_document` and supply it with a TreeSink implementation (RcDom).
        let mut filing = DomifiedFiling::new(filing_contents);
        filing
    }

    #[test]
    fn test_income_statement_known_header_examples() {
        for each in &INCOME_STMT_HEADER_INNER_HTML {
            assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(&each));
        }
    }

    #[test]
    fn test_income_statement_header_location_is_found() {
        for filer in &STATIC_10_Q_FILING_NAMES {
            let mut filing = make_struct(filer);
            filing.start_walker();
            assert!(
                filing.path_to_income_statement_node != None,
                "There should be at least one income statement header in every document!"
            );
        }
    }

    #[test]
    fn test_income_statement_header_location_is_correct() {
        let mut i = 0;
        for filer in &STATIC_10_Q_FILING_NAMES {
            let expected_node_contents = INCOME_STMT_HEADER_INNER_HTML[i];
            let mut filing = make_struct(filer);
            filing.start_walker();
            filing.get_income_statement_node();
            i += 1;
        }
    }
}
