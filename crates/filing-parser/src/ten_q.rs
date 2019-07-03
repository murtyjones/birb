// standard library / core
use core::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

// html parsing
use html5ever::driver::parse_document;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::{SliceExt, TendrilSink};
use html5ever::tree_builder::Attribute;
use html5ever::QualName;

// regex / text matching
use crate::matching_attributes::get_matching_attrs;
use crate::regexes::income_statement_header::INCOME_STATEMENT_HEADER_REGEX;
use crate::regexes::income_statement_table::{INTEREST_INCOME_REGEX, SHARES_OUTSTANDING_REGEX};

// helpers
use crate::helpers::{
    add_attribute, create_x_birb_attr, get_parents_and_indexes, tendril_to_string,
};

pub const MAX_LEVELS_UP: i32 = 4;
// TODO: In the actual rendering of a document, this looks like it should only be a few levels over.
// However when html5ever parses it into a dom, 8 levels over is required. Could just be because of text nodes,
// but it's worth ensuring that there isn't whitespace or something being converted to a node unneccesarily.
pub const MAX_LEVELS_OVER: i32 = 10;

pub struct ProcessedFiling {
    pub dom: RcDom,
    pub income_statement_table_node: Option<Handle>,
    pub income_statement_header_node: Option<Handle>,
    pub income_statement_table_heuristic_found: bool,
}

pub enum ProcessingStep {
    IncomeStatement,
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
            income_statement_table_node: None,
            income_statement_header_node: None,
            income_statement_table_heuristic_found: false,
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
        self.process_step(&doc, &ProcessingStep::IncomeStatement);
        if self.income_statement_header_node.is_none() {
            return Err(ProcessingError::NoIncomeStatementFound {
                cik: String::from("fake"),
            });
        }
        // TODO add other processing steps here
        Ok(())
    }

    fn process_step(&mut self, handle: &Handle, s: &ProcessingStep) {
        match s {
            ProcessingStep::IncomeStatement => {
                self.maybe_find_income_statement_table(handle);
            }
        }
    }

    fn maybe_find_income_statement_table(&mut self, handle: &Handle) {
        let node = handle;
        // If the income statement was already found, exit
        if self.income_statement_table_node.is_some() {
            return ();
        }
        // try to find the nearby income statement table
        self.process_income_statement_if_matching_node_type(handle);
        // If header + table was found, exit
        if self.income_statement_table_node.is_some() {
            return ();
        }
        self.next_iteration(node, &ProcessingStep::IncomeStatement);
    }

    fn next_iteration(&mut self, handle: &Handle, s: &ProcessingStep) {
        for child in handle
            .children
            .borrow()
            .iter()
            .filter(|child| match child.data {
                NodeData::Text { .. } | NodeData::Element { .. } => true,
                _ => false,
            })
        {
            &self.process_step(child, &s);
        }
    }

    fn process_income_statement_if_matching_node_type(&mut self, handle: &Handle) {
        match handle.data {
            NodeData::Text { .. } | NodeData::Element { .. } => {
                self.analyze_node_as_possible_income_statement(handle);
            }
            _ => {}
        }
    }

    fn analyze_node_as_possible_income_statement(&mut self, handle: &Handle) {
        if !self.hueristical_income_statement_content_match(handle) {
            return ();
        };
        let parents_and_indexes = get_parents_and_indexes(handle);

        // for each parent, check if a sibling near to the current child is a table element.
        // if any are, return true.
        for each in &parents_and_indexes {
            let parent = &Rc::clone(&each.0);
            self.node_is_income_statement_table_element(parent);
            let child_index = each.1.clone();
            for sibling_index in 1..=MAX_LEVELS_OVER {
                if self.income_statement_table_node.is_none() {
                    self.offset_node_is_a_table_element(parent, child_index, sibling_index);
                }
            }
        }

        self.maybe_attach_income_statement_attributes(handle, parents_and_indexes);
    }

    fn hueristical_income_statement_content_match(&self, handle: &Handle) -> bool {
        // if a text node with matching regex, return true
        if let NodeData::Text { ref contents } = handle.data {
            let contents = tendril_to_string(contents);
            if INCOME_STATEMENT_HEADER_REGEX.is_match(contents.as_str()) {
                return true;
            }
        }

        // if an element node with matching attributes, return true
        if let NodeData::Element { ref attrs, .. } = handle.data {
            let length = attrs.borrow().len();
            for i in 0..length {
                let attr: &Attribute = &attrs.borrow()[i];
                let matching_attrs = get_matching_attrs();
                for j in 0..matching_attrs.len() {
                    let value = &RefCell::new(attr.value.clone());
                    let matching_attr = &matching_attrs[j];
                    let name_matches = &attr.name.local == matching_attr.name;
                    let value_matches = tendril_to_string(value) == matching_attr.value;
                    if name_matches && value_matches {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn offset_node_is_a_table_element(
        &mut self,
        parent: &Handle,
        child_index: i32,
        sibling_offset: i32,
    ) {
        let sibling_index_from_parent = child_index + sibling_offset;
        let children = &parent.children.borrow();
        // There may not be a sibling at the offset specified, in which case
        // we return "false"
        if (children.len() as i32 - 1) < sibling_index_from_parent as i32 {
            return ();
        }
        let sibling = &children[sibling_index_from_parent as usize];
        self.recursive_node_is_income_statement_table_element(sibling);
    }

    fn recursive_node_is_income_statement_table_element(&mut self, node: &Handle) {
        if self.income_statement_table_node.is_some() {
            return ();
        }

        self.node_is_income_statement_table_element(node);

        for child in node
            .children
            .borrow()
            .iter()
            .filter(|child| match child.data {
                NodeData::Element { .. } => true,
                _ => false,
            })
        {
            self.recursive_node_is_income_statement_table_element(child);
        }
    }

    fn node_is_income_statement_table_element(&mut self, handle: &Handle) {
        if let NodeData::Element { ref name, .. } = handle.data {
            // Should be named <table ...>
            if &name.local == "table" {
                // should have "months ended" somewhere in the table
                self.has_months_ended(handle);
                if self.borrow_mut().income_statement_table_heuristic_found {
                    self.borrow_mut().income_statement_table_node = Some(Rc::clone(handle));
                    return ();
                }
            }
        }
    }

    fn has_months_ended(&mut self, handle: &Handle) {
        if let NodeData::Text { ref contents, .. } = handle.data {
            let contents_str = tendril_to_string(contents);
            // if any of these are discovered, we can feel confident that
            // we have found a table that contains income statement
            // data, as opposed to some other table, and mark the
            if SHARES_OUTSTANDING_REGEX.is_match(contents_str.as_ref())
                || INTEREST_INCOME_REGEX.is_match(contents_str.as_ref())
            {
                self.borrow_mut().income_statement_table_heuristic_found = true;
                return ();
            }
        }
        for child in handle
            .children
            .borrow()
            .iter()
            .filter(|child| match child.data {
                _ => true,
            })
        {
            self.has_months_ended(child);
        }
    }

    fn maybe_attach_income_statement_attributes(
        &mut self,
        handle: &Handle,
        parents_and_indexes: Vec<(Rc<Node>, i32)>,
    ) {
        if self.income_statement_table_node.is_some() {
            self.borrow_mut().income_statement_header_node = Some(handle.clone());
            // If table was found, attach TEMPORARY red background to immediate parent
            // Add the custom style attribute (TODO remove this eventually):
            let colorizer: Attribute = Attribute {
                name: QualName::new(None, ns!(), local_name!("style")),
                value: "background-color: red;".to_tendril(),
            };
            let immediate_parent = &parents_and_indexes[0].0;
            let table_node = &self.income_statement_table_node.as_ref().unwrap();
            add_attribute(immediate_parent, colorizer.clone(), Some("style"));
            add_attribute(table_node, colorizer.clone(), Some("style"));

            // add custom birb income statement identifier
            add_attribute(
                table_node,
                create_x_birb_attr("x-birb-income-statement-table"),
                None,
            );
        }
    }
}

impl ProcessedFiling {
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
    use crate::test_files::MatchType;
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
                panic!("{}", e);
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
            let output_path = String::from(format!("./examples/10-Q/output/{}.html", i));
            processed_filing.write_file_contents(&output_path);
            assert!(
                processed_filing.income_statement_table_node.is_some(),
                "There should be a table for each income statement!"
            );
            assert!(
                processed_filing.income_statement_header_node.is_some(),
                "There should be a header node for each income statement!"
            );
        }
    }

    #[test]
    fn test_income_statement_header_regex_is_correct() {
        let files = get_files();
        for (i, file) in files.iter().enumerate() {
            // Arrange
            if file.match_type == MatchType::Regex {
                let processed_filing = make_processed_filing(file.path);
                let node = processed_filing.income_statement_header_node.unwrap();
                match node.data {
                    NodeData::Text { ref contents } => {
                        let mut stringified_contents = String::new();
                        stringified_contents.push_str(&contents.borrow());
                        assert_eq!(file.header_inner_html.unwrap(), stringified_contents);
                    }
                    _ => panic!("Wrong node!"),
                }
            }
        }
    }
}
