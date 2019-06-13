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
    pub income_statement_node: Option<Handle>,
}

impl DomifiedFiling {
    fn new(filing_contents: String) -> DomifiedFiling {
        DomifiedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(filing_contents),
            path_to_income_statement_node: None,
            income_statement_node: None,
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
                let is_viable_node = self.is_income_statement_header_node(&text);
                let is_node_available = self.borrow_mut().path_to_income_statement_node == None;
                if is_viable_node && is_node_available {
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

    fn set_income_statement_node(&mut self) {
        match &self.path_to_income_statement_node {
            Some(path) => {
                let doc = self.get_doc();
                self.get_node_location(&doc, path.to_owned().borrow_mut());
            }
            None => panic!("Can't get income statement node if none was found!"),
        }
    }

    fn get_node_location(&mut self, handle: &Handle, path: &[i32]) {
        if path.len() == 0 {
            self.borrow_mut().income_statement_node = Some(handle.clone());
            ()
        } else {
            let i = &path[0];
            let child = &handle.children.borrow()[*i as usize];
            self.get_node_location(child, &path[1..]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    struct StaticFile {
        filing_name: String,
        header_inner_html: String,
    }

    lazy_static! {
        static ref FILES: Vec<StaticFile> = vec![
            StaticFile {
                filing_name: String::from("./examples/0001193125-18-037381.txt"),
                header_inner_html: String::from("Consolidated Statements of Income (Loss) "),
            },
            StaticFile {
                filing_name: String::from("./examples/0001000623-17-000125.txt"),
                header_inner_html: String::from("CONDENSED CONSOLIDATED STATEMENTS OF INCOME"),
            },
            StaticFile {
                filing_name: String::from("./examples/0001437749-16-025027.txt"),
                header_inner_html: String::from(
                    "CONDENSED CONSOLIDATED STATEMENTS OF OPERATIONS AND COMPREHENSIVE LOSS"
                ),
            },
            StaticFile {
                filing_name: String::from("./examples/0001004434-17-000011.txt"),
                header_inner_html: String::from("CONSOLIDATED STATEMENTS OF INCOME"),
            },
            StaticFile {
                filing_name: String::from("./examples/0001185185-16-005721.txt"),
                header_inner_html: String::from("CONSOLIDATED STATEMENTS OF OPERATIONS"),
            },
        ];
    }

    fn get_file_contents(filing_name: &String) -> String {
        let path = Path::new(filing_name.as_str());
        let mut file = File::open(path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't get file contents");
        contents
    }

    fn make_struct(filing_name: &String) -> DomifiedFiling {
        let filing_contents = get_file_contents(filing_name);
        // To parse a string into a tree of nodes, we need to invoke
        // `parse_document` and supply it with a TreeSink implementation (RcDom).
        let mut filing = DomifiedFiling::new(filing_contents);
        filing
    }

    #[test]
    fn test_income_statement_known_header_examples() {
        for i in 0..FILES.len() {
            let file = &FILES[i];
            assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(&file.header_inner_html));
        }
    }

    #[test]
    fn test_income_statement_header_location_is_found() {
        for i in 0..FILES.len() {
            let file = &FILES[i];
            let mut filing = make_struct(&file.filing_name);
            filing.start_walker();
            assert!(
                filing.path_to_income_statement_node != None,
                "There should be at least one income statement header in every document!"
            );
        }
    }

    #[test]
    fn test_income_statement_header_location_is_correct() {
        for i in 0..FILES.len() {
            let file = &FILES[i];
            let mut filing = make_struct(&file.filing_name);
            filing.start_walker();
            filing.set_income_statement_node();
            let node = filing.income_statement_node.unwrap();
            match node.data {
                NodeData::Text { ref contents } => {
                    let mut c = String::new();
                    c.push_str(&contents.borrow());
                    assert_eq!(file.header_inner_html, c);
                }
                _ => panic!("Wrong node!"),
            }
        }
    }
}
