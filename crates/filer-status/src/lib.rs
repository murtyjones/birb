//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.

#![deny(missing_docs)]
#[cfg(test)] extern crate mockers_derive;
#[cfg(test)] use mockers_derive::mocked;

extern crate bson;

/// A filer's status
#[cfg_attr(test, mocked)]
pub trait Status {
    /// Tells whether or not the filer is actively submitting filings
    fn is_active(&mut self, cik: String) -> bool;
}

/// Get the status for a filer given the cik
pub fn get_filer_status(cik: String, cond: &mut Status) -> bool {
    cond.is_active(cik)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockers::Scenario;
    #[test]
    fn test_get_status() {
        let cik = String::from("0000000000");
        let scenario = Scenario::new();
        let mut cond = scenario.create_mock_for::<Status>();
        scenario.expect(cond.is_active_call(cik.clone()).and_return(true));
        let r = get_filer_status(cik, &mut cond);
        assert_eq!(r, true);
    }
}
