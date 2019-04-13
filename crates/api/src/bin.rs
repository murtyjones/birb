//! Entrypoint for the Rocket server

#![deny(missing_docs)]

extern crate api_lib;

use api_lib::launch;

/// Launch the server
pub fn main() {
    launch();
}
