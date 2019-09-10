use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Finds a filing that has been collected, but not yet split.
///     1. Splits it
///     2. Compresses its parts
///     3. Uploads them to S3
///     4. Inserts records into `split_filings` to indicate that they were uploaded
fn main() {
    // Spawn 10 threads of the worker per second. Since network-bound,
    // This should be okay in terms of CPU load.
    // Create channels for sending and receiving
    let (tx_0, rx_0) = channel();
    let (tx_1, rx_1) = channel();
    let (tx_2, rx_2) = channel();
    let (tx_3, rx_3) = channel();
    let (tx_4, rx_4) = channel();
    let (tx_5, rx_5) = channel();
    let (tx_6, rx_6) = channel();
    let (tx_7, rx_7) = channel();
    let (tx_8, rx_8) = channel();
    let (tx_9, rx_9) = channel();

    // Spawn one second timer
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        tx_0.send("next iteration").unwrap();
        tx_1.send("next iteration").unwrap();
        tx_2.send("next iteration").unwrap();
        tx_3.send("next iteration").unwrap();
        tx_4.send("next iteration").unwrap();
        tx_5.send("next iteration").unwrap();
        tx_6.send("next iteration").unwrap();
        tx_7.send("next iteration").unwrap();
        tx_8.send("next iteration").unwrap();
        tx_9.send("next iteration").unwrap();
    });

    loop {
        let _ = rx_0.try_recv().map(|_message| _main());
        let _ = rx_1.try_recv().map(|_message| _main());
        let _ = rx_2.try_recv().map(|_message| _main());
        let _ = rx_3.try_recv().map(|_message| _main());
        let _ = rx_4.try_recv().map(|_message| _main());
        let _ = rx_5.try_recv().map(|_message| _main());
        let _ = rx_6.try_recv().map(|_message| _main());
        let _ = rx_7.try_recv().map(|_message| _main());
        let _ = rx_8.try_recv().map(|_message| _main());
        let _ = rx_9.try_recv().map(|_message| _main());
    }
}

/// Performs the actual work of collecting, splitting, uploading, and writing to DB:
fn _main() {
    // Collect:
    // let filing = collect_random_filing();

    // Split:
    // let Vec<SplitFiling> = split_full_submission(filing);

    // Upload split filings to S3
    // upload__to_s3(Vec<SplitFiling>).expect("Couldn't upload to S3");

    // Update DB
    // for each in Vec<SplitFiling>:
    //     tx.insert_row(each);
}
