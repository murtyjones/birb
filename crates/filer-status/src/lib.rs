//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.

#![deny(missing_docs)]

extern crate api_lib;
extern crate bson;

use crate::api_lib::models::filer::Model as Filer;

/// Entrypoint to run the script against a given entity
pub fn main(f: &Filer) -> &Filer {
    f
}

#[cfg(test)]
mod test {
    use super::main;
    use super::Filer;
    #[test]
    fn returns_filer() {
        let cik = String::from("0000000000");
        let mut names = Vec::new();
        // TODO this doesn't seem right:
        names.push(bson::to_bson("marty").unwrap());
        names.push(bson::to_bson("martin").unwrap());
        let mock_filer = Filer { cik, names };
        let r = main(&mock_filer);
        assert_eq!(r.cik, mock_filer.cik);
        assert_eq!(r.names, mock_filer.names);
    }
}
