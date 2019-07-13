// standard library / core
use core::borrow::BorrowMut;
use regex::Regex;
use std::rc::Rc;

// html parsing
use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::{SliceExt, TendrilSink};
use html5ever::tree_builder::Attribute;
use html5ever::QualName;

// regex / text matching
use crate::regexes::statement_of_operations::INCOME_STATEMENT_MIN_REQUIRED_REGEXES;
use crate::regexes::statement_of_operations::INCOME_STATEMENT_REGEXES;

// helpers
use crate::helpers::{
    add_attribute, bfs_no_return, bfs_with_matches, create_x_birb_attr, tendril_to_string,
};

// excluded companies
use crate::excluded_companies::EXCLUDED_COMPANIES;

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
pub trait TableTypeIdentifier {
    fn dom(&self) -> &RcDom;

    fn income_statement_table_nodes(&self) -> &Vec<Handle>;

    fn push_to_income_statement_table_nodes(&mut self, handle: Handle);

    fn filing_contents(&self) -> &String;

    fn filing_key(&self) -> &String;

    /// Gets the Node containing the entire parsed document
    fn get_doc(&self) -> Rc<Node> {
        Rc::clone(&self.dom().document)
    }

    fn probably_find_income_statement(&mut self) -> Result<(), Vec<ProcessingError>> {
        let mut errors = vec![];

        // Process the filing
        let result = self.process();
        if let Err(e) = result {
            errors.push(e);
        }

        /*
         * if there are errors in finding expected tables, check
         * whether or not the filing contains the CIK of a company
         * that is known to not contain those tables. Some companies
         * don't include an income statement, for example. If the
         * filer isn't in this whitelist, return the errors.
         *
         * See: https://www.sec.gov/Archives/edgar/data/1003815/000100381516000011/b4assignorcorp121510k.htm
         */
        if errors.len() > 0 {
            if let Some(_) = EXCLUDED_COMPANIES.iter().find(|&ex_company| {
                self.filing_contents().contains(ex_company.cik)
                    || ex_company.excludable_name.is_match(self.filing_contents())
            }) {
                return Ok(());
            } else {
                return Err(errors);
            }
        }

        Ok(())
    }

    fn process(&mut self) -> Result<(), ProcessingError> {
        let doc = self.get_doc();

        // Find the income statement
        bfs_no_return(doc, |n| self.find_income_statement_or_statements(&n));

        if self.income_statement_table_nodes().len() == 0 {
            return Err(ProcessingError::NoIncomeStatementFound {
                cik: self.filing_key().clone(),
            });
        }

        Ok(())
    }

    fn find_income_statement_or_statements(&mut self, handle: &Handle) -> bool {
        if self.node_is_income_statement_table(handle) {
            println!("Found!");
            self.push_to_income_statement_table_nodes(Rc::clone(handle));
            let index = self.borrow_mut().income_statement_table_nodes().len() as i32 - 1;
            self.attach_income_statement_attributes(&Rc::clone(handle), index);
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

                if count >= INCOME_STATEMENT_MIN_REQUIRED_REGEXES {
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

    fn attach_income_statement_attributes(&mut self, handle: &Handle, index: i32) {
        // If table was found, attach TEMPORARY red background to immediate parent
        // Add the custom style attribute (TODO remove this eventually):
        let colorizer: Attribute = Attribute {
            name: QualName::new(None, ns!(), local_name!("style")),
            value: "background-color: red;".to_tendril(),
        };
        add_attribute(handle, colorizer.clone(), Some("style"));
        // add custom birb income statement identifier
        add_attribute(
            handle,
            create_x_birb_attr("x-birb-earnings-table", index),
            None,
        );
    }
}
