// standard library / core
use core::borrow::BorrowMut;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

// html parsing
use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::{SliceExt, TendrilSink};
use html5ever::tree_builder::Attribute;
use html5ever::QualName;

// regex / text matching
use crate::regexes::statement_of_operations::INCOME_STATEMENT_REGEXES;

// helpers
use crate::helpers::{
    add_attribute, bfs, bfs_no_return, bfs_with_matches, create_x_birb_attr, tendril_to_string,
};

pub struct ProcessedFiling {
    pub dom: RcDom,
    pub income_statement_table_nodes: Vec<Handle>,
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
            income_statement_table_nodes: vec![],
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
        bfs_no_return(doc, |n| self.find_income_statement_or_statements(&n));

        if self.income_statement_table_nodes.len() == 0 {
            return Err(ProcessingError::NoIncomeStatementFound {
                cik: String::from("fake"),
            });
        }
        // TODO add other processing steps here (e.g. balance sheet)
        Ok(())
    }

    fn find_income_statement_or_statements(&mut self, handle: &Handle) -> bool {
        if self.node_is_income_statement_table(handle) {
            println!("Found!");
            self.borrow_mut()
                .income_statement_table_nodes
                .push(Rc::clone(handle));
            self.attach_income_statement_attributes();
            return true;
        };
        false
    }

    fn node_is_income_statement_table(&mut self, handle: &Handle) -> bool {
        if let NodeData::Element { ref name, .. } = handle.data {
            // Should be named <table ...>
            if &name.local == "table" {
                let cb = |n| self.table_regex_match(&n);
                let count = bfs_with_matches(Rc::clone(handle), cb);

                /*
                 * There should be at least 2 regex matches that indicate
                 * that this is an income statement. If less, return false.
                 * This number should get larger over time as the regex patterns
                 * become more accurate. If you find yourself lowering it...
                 * Think about whether that is the right thing to do.
                 */
                const MIN_REQUIRED_MATCHES: i32 = 4;

                if count >= MIN_REQUIRED_MATCHES {
                    return true;
                }
            }
        }
        false
    }

    /// if any of these patterns are discovered, we can feel confident
    /// that we have found a table that contains income statement data,
    /// as opposed to some other table, and mark the
    fn table_regex_match(&mut self, handle: &Handle) -> Option<&'static Regex> {
        if let NodeData::Text { ref contents, .. } = handle.data {
            let contents_str = tendril_to_string(contents);

            for regex in INCOME_STATEMENT_REGEXES.iter() {
                if regex.is_match(contents_str.as_ref()) {
                    return Some(regex);
                }
            }

            return None;
        }
        None
    }

    fn attach_income_statement_attributes(&mut self) {
        // If table was found, attach TEMPORARY red background to immediate parent
        // Add the custom style attribute (TODO remove this eventually):
        let colorizer: Attribute = Attribute {
            name: QualName::new(None, ns!(), local_name!("style")),
            value: "background-color: red;".to_tendril(),
        };
        for (i, each) in self.income_statement_table_nodes.iter().enumerate() {
            add_attribute(each, colorizer.clone(), Some("style"));
            // add custom birb income statement identifier
            add_attribute(
                each,
                create_x_birb_attr("x-birb-income-statement-table", i as i32),
                None,
            );
        }
    }
}

impl ProcessedFiling {
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

    fn make_processed_filing(path: &'static str) -> ProcessedFiling {
        let filing_contents = get_file_contents(path);
        // To parse a string into a tree of nodes, we need to invoke
        // `parse_document` and supply it with a TreeSink implementation (RcDom).
        let processed_filing = ProcessedFiling::new(filing_contents);
        match processed_filing {
            Ok(p_f) => p_f,
            Err(e) => {
                panic!("[{}] {}", path, e);
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
        let files = get_files();
        for (i, file) in files.iter().enumerate() {
            let mut processed_filing = make_processed_filing(file.path);
            assert!(
                processed_filing.income_statement_table_nodes.len() > 0,
                "There should be at least one table for each income statement!"
            );

            let stringified_result = processed_filing.get_doc_as_str();
            //            assert!(
            //                stringified_result.contains(file.table_element),
            //                "Table element expected content was not found!"
            //            );
            println!(
                "Table count: {}",
                processed_filing.income_statement_table_nodes.len()
            );
            let output_path = String::from(format!("./examples/10-Q/output/{}.html", i));
            std::fs::write(output_path, stringified_result).expect("Unable to write file");
        }
    }
}
