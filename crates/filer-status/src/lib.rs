//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.

#![deny(missing_docs)]

extern crate api_lib;
extern crate bson;
extern crate failure;
#[macro_use]
extern crate html5ever;
extern crate reqwest;

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
use std::default::Default;
use std::string::String;

use api_lib::models::filer::Model as Filer;
use html5ever::driver::parse_document;
use html5ever::driver::ParseOpts;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::serialize::serialize;
use tendril::stream::TendrilSink;

#[cfg(test)] use std::fs;
#[cfg(test)] use std::path::Path;

#[cfg(test)]
const MOCK_INACTIVE_FILER_CIK: &'static str = "0000948605"; // Kenneth Sawyer
#[cfg(test)]
const MOCK_ACTIVE_FILER_CIK: &'static str = "0001318605"; // Tesla, Inc.

/// Filing status of the filer
pub trait FilingStatus {
    /// Is the filer active in filing with the SEC?
    fn is_active(&self) -> bool;
    /// Gets the doc from sec.gov
    fn get_10q_doc(&self) -> Result<String, reqwest::Error>;
    /// Parses the html
    fn generate_dom(&self, html: String) -> RcDom;
    /// Finds the div
    fn walk_dom_find_div(&self, handle: Handle) -> bool;
}

/// Implements the status retrieval for the Filer model
impl FilingStatus for Filer {
    fn is_active(&self) -> bool {
        true
    }

    #[cfg(not(test))] // TODO use "failure" crate instead of reqwest::Error
    fn get_10q_doc(&self) -> Result<String, reqwest::Error> {
        let url: &str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0001318605&type=10-Q&dateb=&owner=include&count=40";
        reqwest::get(url)?.text()
    }

    #[cfg(test)] // TODO use "failure" crate instead of reqwest::Error
    fn get_10q_doc(&self) -> Result<String, reqwest::Error> {
        let mut filer_mock_html_path: String = "../../seed-data/test".to_string();
        match &*self.cik {
            MOCK_INACTIVE_FILER_CIK => {
                filer_mock_html_path =
                    filer_mock_html_path + &"/kenneth-sawyer-10q-listings".to_string()
            }
            MOCK_ACTIVE_FILER_CIK => {
                filer_mock_html_path = filer_mock_html_path + &"/tsla-10q-listings".to_string()
            }
            _ => {
                // Just use an active filer if no match
                filer_mock_html_path = filer_mock_html_path + &"/tsla-10q-listings".to_string()
            }
        }
        // TODO: This path is relative to birb/crates/filer-status. Not sure whether it
        // holds up at release compile time but I guess since this is a test it's okay.
        let path: &Path = Path::new(&filer_mock_html_path);
        let html: String = fs::read_to_string(path).unwrap();
        Ok(html)
    }

    fn generate_dom(&self, html: String) -> RcDom {
        let sink: RcDom = RcDom::default();
        let opts: ParseOpts = ParseOpts::default();
        parse_document(sink, opts).from_utf8().one(html.as_bytes())
    }

    fn walk_dom_find_div(&self, handle: Handle) -> bool {
        let node = handle;
        let mut has_10q_input: bool = false;
        match node.data {
            NodeData::Document => println!("#Document"),

            NodeData::Doctype {
                ref name,
                ref public_id,
                ref system_id,
            } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

            NodeData::Text { ref contents } => {
                println!("#text: {}", escape_default(&contents.borrow()))
            }

            NodeData::Comment { ref contents } => println!("<!-- {} -->", escape_default(contents)),

            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                assert!(name.ns == ns!(html));
                print!("<{}", name.local);
                if name.local.to_string() == "input" {
                    for attr in attrs.borrow().iter() {
                        assert!(attr.name.ns == ns!());
                        print!(" {}=\"{}\"", attr.name.local, attr.value);
                        if attr.name.local.to_string() == "value"
                            && attr.value.to_string() == "10-Q"
                        {
                            has_10q_input = true;
                        }
                    }
                }
                println!(">");
            }

            NodeData::ProcessingInstruction { .. } => unreachable!(),
        }

        if has_10q_input == true {
            println!("returning {} ", has_10q_input);
            true
        }

        for child in node.children.borrow().iter() {
            self.walk_dom_find_div(child.clone());
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_mock_filer(cik: &'static str) -> Filer {
        let cik = String::from(cik);
        let mut names = vec![];
        names.push(bson::to_bson("alias 1").unwrap());
        names.push(bson::to_bson("alias 2").unwrap());
        Filer { cik, names }
    }

    #[test]
    fn test_is_active() {
        // Arrange
        let f: Filer = get_mock_filer(MOCK_INACTIVE_FILER_CIK);

        // Assert
        let r = f.is_active();

        // Act
        assert_eq!(r, true)
    }

    #[test]
    fn test_get_10q_doc() {
        // Arrange
        let filer_inactive: Filer = get_mock_filer(MOCK_INACTIVE_FILER_CIK);
        let filer_inactive_path: &Path =
            Path::new("../../seed-data/test/kenneth-sawyer-10q-listings");
        let filer_inactive_expected_html = fs::read_to_string(filer_inactive_path);
        let filer_active: Filer = get_mock_filer(MOCK_ACTIVE_FILER_CIK);
        let filer_active_path: &Path = Path::new("../../seed-data/test/tsla-10q-listings");
        let filer_active_expected_html = fs::read_to_string(filer_active_path);

        // Assert
        let filer_inactive_result = filer_inactive.get_10q_doc();
        let filer_active_result = filer_active.get_10q_doc();

        // Act
        assert_eq!(
            filer_inactive_result.unwrap(),
            filer_inactive_expected_html.unwrap()
        );
        assert_eq!(
            filer_active_result.unwrap(),
            filer_active_expected_html.unwrap()
        );
    }

    #[test]
    fn test_generate_dom() {
        // Arrange
        let f: Filer = get_mock_filer(MOCK_ACTIVE_FILER_CIK);
        let html = f.get_10q_doc();
        match html {
            Ok(content) => {
                // Act
                let dom = f.generate_dom(content);

                // Assert
                let mut serialized = Vec::new();
                serialize(&mut serialized, &dom.document, Default::default()).unwrap();
                assert_eq!(
                    String::from_utf8(serialized)
                        .unwrap()
                        .contains("<input type=\"hidden\" name=\"CIK\" value=\"0001318605\">"),
                    true
                );
            }
            Err(_) => panic!("get_10q_doc: error getting 10q doc"),
        }
    }

    #[test]
    fn test_walk_dom_find_div() {
        // Arrange
        let f: Filer = get_mock_filer(MOCK_ACTIVE_FILER_CIK);
        let html: String = f.get_10q_doc().unwrap();
        let dom: RcDom = f.generate_dom(html);

        // Act
        let r: bool = f.walk_dom_find_div(dom.document);
        assert_eq!(r, true);
        panic!()
    }
}
