//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.

#![deny(missing_docs)]

extern crate api_lib;
extern crate bson;
extern crate failure;
extern crate reqwest;

use api_lib::models::filer::Model as Filer;

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
        let mut filer_mock_html_path: &'static str = "";
        if self.cik == INACTIVE_FILER_CIK {
            filer_mock_html_path = "../../seed-data/unit-test/kenneth-sawyer-10q-listings";
        } else if self.cik == ACTIVE_FILER_CIK {
            filer_mock_html_path = "../../seed-data/unit-test/tsla-10q-listings";
        }
        // TODO: This path is relative to birb/crates/filer-status. Not sure whether it
        // holds up at compile time but I guess since this is a test it's okay.
        let path: &Path = Path::new(filer_mock_html_path);
        let html: String = fs::read_to_string(path)?;
        Ok(html)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_filer_status() {
        // Arrange
        let cik = String::from("0000000000");
        let mut names = vec![];
        names.push(bson::to_bson("marty").unwrap());
        names.push(bson::to_bson("martin").unwrap());
        let f = Filer { cik, names };

        // Assert
        let r = f.is_active();

        // Act
        assert_eq!(r, true)
    }

    #[test]
    fn test_get_10q_listings_inactive_filer() {
        // Arrange
        let cik = String::from(INACTIVE_FILER_CIK);
        let mut names = vec![];
        names.push(bson::to_bson("ken sawyer").unwrap());
        names.push(bson::to_bson("kenneth").unwrap());
        let f = Filer { cik, names };

        // Assert
        let r = f.get_10q_doc();

        // Act
        // assert_eq!(r, Ok(()))
    }
}
