extern crate api_lib;
extern crate filing_data;
extern crate filing_metadata;
#[macro_use]
extern crate log;
extern crate env_logger;
use filing_data::main as get_one_filing;
use postgres::{Connection, TlsMode};
use std::env;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Find a filer with no status and update it.
pub fn main() -> () {
    // Initialize logging
    env_logger::init();

    // Make one request to SEC.gov every 2 seconds
    const SECONDS_DELAY: u16 = 2;

    // Create channels for sending and receieving
    let (one_tx, one_rx) = channel();

    // Spawn one second timer
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(SECONDS_DELAY.into()));
        one_tx.send("next iteration").unwrap();
    });

    loop {
        let _ = one_rx.try_recv().map(|_message| {
            get_one_filing();
        });
    }
}
