//! Returns information about a given filer's status

#![deny(missing_docs)]

/// API is used for models and db connectivity
extern crate api_lib;

/// Entrypoint to run the script against a given entity
pub fn main() {
    return api_lib::launch();
}
