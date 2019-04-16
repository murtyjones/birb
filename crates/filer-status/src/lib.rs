//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.

#![deny(missing_docs)]

extern crate api_lib;
extern crate bson;
extern crate failure;
extern crate reqwest;

use api_lib::models::filer::Model as Filer;

/// Filing status of the filer
pub trait FilingStatus {
    /// Is the filer active in filing with the SEC?
    fn is_active(&self) -> bool;
    /// Gets the doc from sec.gov
    fn get_10q_doc(&self) -> Result<(), Box<std::error::Error>>;
}

/// Implements the status retrieval for the Filer model
impl FilingStatus for Filer {
    fn is_active(&self) -> bool {
        true
    }

    #[cfg(not(test))] // TODO use failure library instead of Box<...>
    fn get_10q_doc(&self) -> Result<(), Box<std::error::Error>> {
        let url = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0001318605&type=10-Q&dateb=&owner=include&count=40";
        let resp: String = reqwest::get(url)?.text()?;
        Ok(())
    }

    #[cfg(test)] // TODO use failure library instead of Box<...>
    fn get_10q_doc(&self) -> Result<(), Box<std::error::Error>> {
        let url = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0001318605&type=10-Q&dateb=&owner=include&count=40";
        let resp: String = reqwest::get(url)?.text()?;
        println!("response body: {}", resp);
        std::fs::write(
            "/Users/murtyjones/birb/seed-data/unit-test/tsla-10q-listings",
            resp,
        )
        .expect("Unable to write file");
        Ok(())
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
    fn test_get_10q() {
        // Arrange
        let cik = String::from("0000000000");
        let mut names = vec![];
        names.push(bson::to_bson("marty").unwrap());
        names.push(bson::to_bson("martin").unwrap());
        let f = Filer { cik, names };

        // Assert
        let r = f.get_10q_doc();

        // Act
        // assert_eq!(r, Ok(()))
    }
}
