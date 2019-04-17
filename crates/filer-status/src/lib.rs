//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.

#![deny(missing_docs)]

extern crate api_lib;
extern crate bson;
extern crate failure;
extern crate html5ever;
extern crate reqwest;

use api_lib::models::filer::Model as Filer;
use html5ever::driver::parse_document;
use html5ever::driver::ParseOpts;
use html5ever::driver::Parser;
use html5ever::rcdom::RcDom;
use html5ever::serialize::serialize;
use tendril::stream::TendrilSink;

#[cfg(test)] use std::fs;
#[cfg(test)] use std::path::Path;

#[cfg(test)]
const INACTIVE_FILER_CIK: &'static str = "0000948605"; // Kenneth Sawyer
#[cfg(test)]
const ACTIVE_FILER_CIK: &'static str = "0001318605"; // Tesla, Inc.

/// Filing status of the filer
pub trait FilingStatus {
    /// Is the filer active in filing with the SEC?
    fn is_active(&self) -> bool;
    /// Gets the doc from sec.gov
    fn get_10q_doc(&self) -> Result<String, Box<std::error::Error>>;
    /// Parses the html
    fn parse_html(&self, html: String) -> RcDom;
}

/// Implements the status retrieval for the Filer model
impl FilingStatus for Filer {
    fn is_active(&self) -> bool {
        true
    }

    #[cfg(not(test))] // TODO use failure library instead of Box<...>
    fn get_10q_doc(&self) -> Result<String, Box<std::error::Error>> {
        let url: &str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0001318605&type=10-Q&dateb=&owner=include&count=40";
        let html: String = reqwest::get(url)?.text()?;
        Ok(html)
    }

    #[cfg(test)] // TODO use failure library instead of Box<...>
    fn get_10q_doc(&self) -> Result<String, Box<std::error::Error>> {
        let mut filer_mock_html_path: String = "../../seed-data/unit-test".to_string();
        match &*self.cik {
            INACTIVE_FILER_CIK => {
                filer_mock_html_path =
                    filer_mock_html_path + &"/kenneth-sawyer-10q-listings".to_string()
            }
            ACTIVE_FILER_CIK => {
                filer_mock_html_path = filer_mock_html_path + &"/tsla-10q-listings".to_string()
            }
            _ => {
                // Just use an active filer if no match
                filer_mock_html_path = filer_mock_html_path + &"/tsla-10q-listings".to_string()
            }
        }
        // TODO: This path is relative to birb/crates/filer-status. Not sure whether it
        // holds up at compile time but I guess since this is a test it's okay.
        let path: &Path = Path::new(&filer_mock_html_path);
        let html: String = fs::read_to_string(path)?;
        Ok(html)
    }

    fn parse_html(&self, html: String) -> RcDom {
        let sink: RcDom = RcDom::default();
        let opts: ParseOpts = ParseOpts::default();
        parse_document(sink, opts).from_utf8().one(html.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_mock_inactive_filer() -> Filer {
        let cik = String::from(INACTIVE_FILER_CIK);
        let mut names = vec![];
        names.push(bson::to_bson("ken sawyer").unwrap());
        names.push(bson::to_bson("kenneth").unwrap());
        Filer { cik, names }
    }

    #[test]
    fn test_get_filer_status() {
        // Arrange
        let f: Filer = get_mock_inactive_filer();

        // Assert
        let r = f.is_active();

        // Act
        assert_eq!(r, true)
    }

    #[test]
    fn test_get_10q_listings_inactive_filer() {
        // Arrange
        let f: Filer = get_mock_inactive_filer();
        let path: &Path = Path::new("../../seed-data/unit-test/kenneth-sawyer-10q-listings");
        let expected_html = fs::read_to_string(path);

        // Assert
        let r = f.get_10q_doc();

        // Act
        assert_eq!(r.unwrap(), expected_html.unwrap())
    }

    #[test]
    fn test_parse_html() {
        let f: Filer = get_mock_inactive_filer();
        let html = String::from("<title>Hello whirled");
        let dom = f.parse_html(html);
        let mut serialized = Vec::new();
        serialize(&mut serialized, &dom.document, Default::default()).unwrap();
        assert_eq!(
            String::from_utf8(serialized).unwrap().replace(" ", ""),
            "<html><head><title>Hellowhirled</title></head><body></body></html>"
        );
    }
}
