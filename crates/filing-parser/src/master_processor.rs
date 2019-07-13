use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, RcDom};
use html5ever::tendril::TendrilSink;
//use std::borrow::BorrowMut;
use std::rc::Rc;

use crate::processing_steps::table_type_identifier::TableTypeIdentifier;
use core::borrow::BorrowMut;

pub struct ParsedFiling {
    pub filing_contents: String,
    pub dom: RcDom,
    pub income_statement_table_nodes: Vec<Handle>,
}

impl TableTypeIdentifier for ParsedFiling {
    fn dom(&self) -> &RcDom {
        &self.dom
    }

    fn income_statement_table_nodes(&self) -> &Vec<Handle> {
        &self.income_statement_table_nodes
    }

    fn push_to_income_statement_table_nodes(&mut self, handle: Handle) {
        &self.income_statement_table_nodes.push(handle);
    }

    fn filing_contents(&self) -> &String {
        &self.filing_contents
    }
}

#[derive(Debug, Fail, PartialEq)]
pub enum ParsingError {
    #[fail(display = "Failed to parse document for CIK: {}", cik)]
    FailedToParse { cik: String },
    #[fail(display = "No income statement found for CIK: {}", cik)]
    NoIncomeStatementFound { cik: String },
}

impl ParsedFiling {
    pub fn new(filing_contents: String) -> Result<ParsedFiling, Vec<ParsingError>> {
        let mut p_f = ParsedFiling {
            filing_contents: filing_contents.clone(),
            dom: parse_document(RcDom::default(), Default::default()).one(&*filing_contents),
            income_statement_table_nodes: vec![],
        };

        let income_statement_result = p_f.probably_find_income_statement().unwrap();

        // Return the processed document
        Ok(p_f)
    }

    /// Gets the Node containing the entire parsed document
    fn get_doc(&self) -> Rc<Node> {
        Rc::clone(&self.dom.document)
    }
}

impl ParsedFiling {
    pub fn get_doc_as_str(&mut self) -> String {
        let doc: &Rc<Node> = &self.get_doc();
        let mut bytes = vec![];
        html5ever::serialize::serialize(
            &mut bytes,
            doc,
            html5ever::serialize::SerializeOpts::default(),
        )
        .expect("Couldn't write to file.");
        String::from_utf8(bytes).unwrap()
    }

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
