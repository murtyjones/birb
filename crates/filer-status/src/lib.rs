//! Returns information about a given filer's status. Intended
//! to be as static as possible - it does make HTTP requests, but
//! should not care about the DB.

#![deny(missing_docs)]

extern crate api_lib;
extern crate bson;
use bson::Array;

use crate::api_lib::models::filer::Model as Filer;

/// Entrypoint to run the script against a given entity
pub fn main(f: Filer) -> Filer {
    f
}

//#[cfg(test)]
//mod test {
//    use super::main;
//    use super::Filer;
//    #[test]
//    fn returns_filer() {
//        let f = Filer {
//            cik: String::from("0000000000"),
//            names: // TODO implmenent a bson array here
//        };
//        let r = main(f);
//        assert_eq!(r, f)
//    }
//}
