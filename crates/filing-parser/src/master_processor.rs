use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, RcDom};
use html5ever::tendril::TendrilSink;
//use std::borrow::BorrowMut;
use std::rc::Rc;

use crate::processing_steps::metadata_remover::{MetadataRemover, ProcessingError};
use crate::processing_steps::table_accessor::TableAccessor;
use crate::processing_steps::table_type_identifier::TableTypeIdentifier;
use core::borrow::BorrowMut;

pub struct ParsedFiling {
    pub filing_contents: String,
    pub filing_key: String,
    pub dom: RcDom,
    pub income_statement_table_nodes: Vec<Handle>,
}

impl TableAccessor for ParsedFiling {
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

    fn filing_key(&self) -> &String {
        &self.filing_key
    }
}

impl TableTypeIdentifier for ParsedFiling {}

impl MetadataRemover for ParsedFiling {}

#[derive(Debug, Fail, PartialEq)]
pub enum ParsingError {
    #[fail(display = "Failed to parse document for filing_key: {}", filing_key)]
    FailedToParse { filing_key: String },
    #[fail(display = "No income statement found for filing_key: {}", filing_key)]
    NoIncomeStatementFound { filing_key: String },
}

impl ParsedFiling {
    pub fn new(
        filing_contents: String,
        object_key: String,
    ) -> Result<ParsedFiling, Vec<ParsingError>> {
        let mut p_f = ParsedFiling {
            filing_contents: filing_contents.clone(),
            filing_key: object_key.clone(),
            dom: parse_document(RcDom::default(), Default::default()).one(&*filing_contents),
            income_statement_table_nodes: vec![],
        };

        p_f.probably_find_income_statement();
        p_f.probably_strip_metadata_nodes();

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::get_abs_path;
    use std::fs::File;
    use std::io::prelude::*;
    // test files
    use crate::test_files::get_files;

    fn get_file_contents(path: &'static str) -> String {
        let path = get_abs_path(&String::from(path));
        let mut file = File::open(path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't get file contents");
        contents
    }

    fn make_processed_filing(path: &'static str) -> ParsedFiling {
        let filing_contents = get_file_contents(path);
        let fake_key = String::from("000/000.txt");
        // To parse a string into a tree of nodes, we need to invoke
        // `parse_document` and supply it with a TreeSink implementation (RcDom).
        let processed_filing = ParsedFiling::new(filing_contents, fake_key);
        match processed_filing {
            Ok(p_f) => p_f,
            Err(errors) => {
                errors.iter().for_each(|error| {
                    println!("[{}] {}", path, error);
                });
                panic!("Failed to process!");
            }
        }
    }

    #[test]
    fn test_should_err_when_no_income_statement_found() {
        let fake_html = String::from("<html></html>");
        let fake_key = String::from("000/000.txt");
        let processed_filing = ParsedFiling::new(fake_html, fake_key);
        assert!(processed_filing.is_err());
        if let Err(errors) = processed_filing {
            assert_eq!(
                errors[0],
                ParsingError::NoIncomeStatementFound {
                    filing_key: String::from("fake")
                }
            );
        }
    }

    #[test]
    fn test_income_statement_header_and_table_location_found() {
        let files = get_files();
        for (i, file) in files.iter().enumerate() {
            let mut processed_filing = make_processed_filing(file.path);
            assert!(
                processed_filing.income_statement_table_nodes.len() > 0,
                "There should be at least one table for each income statement!"
            );

            let stringified_result = processed_filing.get_doc_as_str();
            let output_path = String::from(format!("./examples/10-Q/output/{}.html", i));
            std::fs::write(output_path, stringified_result.clone()).expect("Unable to write file");
            assert!(
                stringified_result.contains(file.table_element),
                "[file: {}] Table element expected content was not found!",
                i
            );
            assert_eq!(
                processed_filing.income_statement_table_nodes.len() as i32,
                file.income_statement_table_count,
                "[file: {}] Should have expected number of tables!",
                i
            );
        }
    }
}
