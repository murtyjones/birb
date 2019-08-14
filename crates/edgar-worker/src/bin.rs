extern crate env_logger;
extern crate filing_data;
extern crate filing_metadata;
extern crate log;
extern crate server_lib;
use filing_data::main as get_one_filing;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Find a filer with no status and update it.
pub fn main() -> () {
    // Initialize logging
    env_logger::init();

    // Create channels for sending and receieving
    let (one_tx, one_rx) = channel();

    // Spawn one second timer
    thread::spawn(move || loop {
        one_tx.send("next iteration").unwrap();
    });

    loop {
        let _ = one_rx.try_recv().map(|_message| {
            get_one_filing();
        });
    }
}
