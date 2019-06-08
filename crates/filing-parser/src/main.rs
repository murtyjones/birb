extern crate regex;
extern crate xml5ever;
#[macro_use]
extern crate lazy_static;

// Std requires
use std::ascii::escape_default;
use std::default::Default;
use std::rc::*;

// xml requires
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::tendril::TendrilSink;
use xml5ever::tree_builder::TreeSink;

// regex requires
use regex::Regex;

fn main() {
    let filing_contents = include_str!("0001193125-18-037381.txt");
    println!("{}", filing_contents.len());
    // To parse a string into a tree of nodes, we need to invoke
    // `parse_document` and supply it with a TreeSink implementation (RcDom).
    let filing = DomifiedFiling::new(filing_contents);
    let node_1 = &filing.get_doc().children.borrow()[0];
    println!("{:?}", node_1);
}

pub struct DomifiedFiling {
    pub dom: RcDom,
    pub income_statement_header_node_count: i32,
}

impl DomifiedFiling {
    fn new(filing_contents: &'static str) -> DomifiedFiling {
        DomifiedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            income_statement_header_node_count: 0,
        }
    }

    fn get_doc(&self) -> &Rc<Node> {
        &self.dom.document
    }

    /// For now, get count of elements that match the regex pattern
    fn get_income_statement_header(&self) -> i32 {
        let doc = self.get_doc();
        let income_statement_expr =
            Regex::new(r"consolidated\s+statements\s+of\s+(income|operations)").unwrap();
    }

    fn is_income_statement_header_node(text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"consolidated\s+statements\s+of\s+(income|operations)").unwrap();
        }
        RE.is_match(text)
    }

    fn walker(&mut self, handle: &Handle) {
        let node = handle;

        match node.data {
            NodeData::Text { ref contents } => {
                let text = escape_default(&contents.borrow());
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

    fn walk(&self, handle: &Handle) {
        let node = handle;

        match node.data {
            NodeData::Document => println!("#document"),

            NodeData::Text { ref contents } => {
                println!("#text {}", escape_default(&contents.borrow()))
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
            &self.walk(child);
        }
    }
}
