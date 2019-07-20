use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, RcDom};
use html5ever::tendril::TendrilSink;
//use std::borrow::BorrowMut;
use std::rc::Rc;

use crate::processing_steps::income_statement_identifier::{
    IncomeStatementIdentifier, ProcessingError as IncomeStatementIdentifierProcessingError,
};
use crate::processing_steps::metadata_remover::{
    MetadataRemover, ProcessingError as MetadataRemoverProcessingError,
};
use crate::processing_steps::table_accessor::TableAccessor;
use core::borrow::BorrowMut;

pub struct ParsedFiling {
    pub filing_contents: String,
    pub filing_key: String,
    pub dom: RcDom,
    pub income_statement_table_nodes: Vec<Handle>,
}

#[derive(Debug, Fail, PartialEq)]
pub enum ParsingError {
    #[fail(display = "Error when identifying table types")]
    IncomeStatementIdentifierProcessingError(IncomeStatementIdentifierProcessingError),
    #[fail(display = "Error when removing metadata")]
    MetadataRemoverProcessingError(MetadataRemoverProcessingError),
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

impl IncomeStatementIdentifier for ParsedFiling {}

impl MetadataRemover for ParsedFiling {}

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

        let mut errors: Vec<ParsingError> = vec![];

        let mut r = p_f.probably_find_income_statement();
        if let Err(e) = r {
            errors.push(ParsingError::IncomeStatementIdentifierProcessingError(e));
        }
        let mut r = p_f.probably_strip_metadata_nodes();
        if let Err(e) = r {
            errors.push(ParsingError::MetadataRemoverProcessingError(e));
        }

        if errors.len() > 0 {
            return Err(errors);
        }

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
    fn test_should_provide_expeted_errors() {
        let fake_html = String::from("<html></html>");
        let fake_key = String::from("000/000.txt");
        let processed_filing = ParsedFiling::new(fake_html, fake_key);
        match processed_filing {
            Ok(_) => panic!("Not expecting okay!"),
            Err(e) => {
                assert_eq!(e.len(), 2);
            }
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

            // earnings identifiers assertions:
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

            // metadata removal assertions:
            assert!(
                !stringified_result.contains("<sec-header"),
                "[file: {}] Contains <sec-header> node(s)!",
                i
            );
            assert!(
                !stringified_result.contains("<xbrl"),
                "[file: {}] Contains <xbrl> nodes!",
                i
            );
        }
    }
}
