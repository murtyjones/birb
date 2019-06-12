extern crate regex;
extern crate xml5ever;
#[macro_use]
extern crate lazy_static;

// Std requires
use std::default::Default;
use std::rc::*;
use std::string::String;

// xml requires
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::tendril::TendrilSink;

// regex requires
use core::borrow::BorrowMut;
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
    pub is_income_statement_header_located: bool,
}

impl DomifiedFiling {
    fn new(filing_contents: String) -> DomifiedFiling {
        DomifiedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            is_income_statement_header_located: false,
        }
    }

    fn get_doc(&self) -> &Rc<Node> {
        &self.dom.document
    }

    fn is_income_statement_header_node(&self, text: &str) -> bool {
        INCOME_STATEMENT_HEADER_REGEX.is_match(text)
    }

    fn walker(&mut self, handle: &Handle) {
        let node = handle;

        match node.data {
            NodeData::Text { ref contents } => {
                let text = contents.borrow();
                if self.is_income_statement_header_node(&text) {
                    println!("\n\n\n{}", &text);
                    println!("\n\n\n{:?}", &node);
                    self.borrow_mut().is_income_statement_header_located = true;
                    ()
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

    // TODO use S3 when running these in CI.
    const STATIC_10_Q_FILING_NAMES: [&'static str; 4] = [
        "./examples/0001193125-18-037381.txt",
        "./examples/0001000623-17-000125.txt",
        "./examples/0001437749-16-025027.txt",
        "./examples/0001004434-17-000011.txt",
    ];

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
    fn test_income_statement_known_header_examples() {
        const INCOME_STMT_HEADER_INNER_HTML: [&'static str; 4] = [
            "Consolidated Statements of Income (Loss) ",
            "CONDENSED CONSOLIDATED STATEMENTS OF INCOME",
            "<b>CONDENSED CONSOLIDATED STATEMENTS OF OPERATIONS AND COMPREHENSIVE LOSS</b>",
            "CONSOLIDATED STATEMENTS OF INCOME",
        ];
        for each in &INCOME_STMT_HEADER_INNER_HTML {
            assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(&each));
        }
    }

    #[test]
    fn test_income_statement_header_is_found() {
        for filer in &STATIC_10_Q_FILING_NAMES {
            let filing = run_for_one(filer);
            assert!(
                filing.is_income_statement_header_located,
                "There should be at least one income statement header in every document!"
            );
        }
    }
}
