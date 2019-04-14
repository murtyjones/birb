//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.
#![feature(proc_macro)]
#![deny(missing_docs)]
extern crate mockers;
extern crate mockers_derive;
use mockers_derive::mocked;

extern crate api_lib;
extern crate bson;

use api_lib::models::filer::Model as Filer;

/// Filing status of the filer
#[mocked]
pub trait FilingStatus {
    /// Is the filer active in filing with the SEC?
    fn is_active(&self) -> bool;
    /// Gets the doc from sec.gov
    fn get_10q_doc(&self) -> String;
}

/// Implements the status retrieval for the Filer model
impl FilingStatus for Filer {
    fn is_active(&self) -> bool {
        self.get_10q_doc();
        true
    }

    fn get_10q_doc(&self) -> String {
        self.cik.clone()
    }
}

/// Wow!
pub fn get_filer_status(f: &mut FilingStatus) -> bool {
    f.is_active()
}

#[cfg(test)]
mod test {
    use super::*;
    use mockers::Scenario;

    #[test]
    fn test_get_filer_status() {
        // Arrange
        let cik = String::from("0000000000");
        let mut names = vec![];
        names.push(bson::to_bson("marty").unwrap());
        names.push(bson::to_bson("martin").unwrap());
        let scenario = Scenario::new();
        let mut mock = scenario.create_mock_for::<FilingStatus>();
        let f = Filer { cik, names };

        // Assert
        scenario.expect(mock.is_active_call().and_return(false));

        // Act
        let r = get_filer_status(&mut mock);
        assert_eq!(r, false)
    }
}
