extern crate env_logger;
extern crate filing_data;
extern crate filing_metadata;
extern crate log;
extern crate server_lib;
use filing_data::get_one_filing;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Find a filer with no status and update it.
pub fn main() -> () {
    // Initialize logging
    env_logger::init();

    // Create channels for sending and receieving
    let (tx_1, rx_1) = channel();
    let (tx_2, rx_2) = channel();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        tx_1.send("next iteration").unwrap();
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        tx_2.send("next iteration").unwrap();
    });

    loop {
        let _ = rx_1.try_recv().map(|_msg| {
            println!("1");
            get_one_filing();
        });
        let _ = rx_2.try_recv().map(|_msg| {
            println!("1");
            get_one_filing();
        });
    }
}
