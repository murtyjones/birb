//! Entrypoint for the Rocket server

#![deny(missing_docs)]

extern crate server_lib;

use server_lib::launch;

/// Launch the server
pub fn main() {
    launch();
}
