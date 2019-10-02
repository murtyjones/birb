use aws::s3::{get_s3_client, store_s3_document_gzipped_async};
use failure;
use filing_parser::split_full_submission::split_full_submission;
use futures::{future, stream, Future, Stream};
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::rows::{Row, Rows};
use postgres::Connection;
use rayon::prelude::*;
use rusoto_core::RusotoFuture;
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use rusoto_s3::{GetObjectError, PutObjectOutput};
use rusoto_s3::{GetObjectOutput, GetObjectRequest};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio_core::reactor::Core;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection};

const NUMBER_TO_COLLECT_PER_ITERATION: i32 = 10;
const PARALLEL_REQUESTS: usize = 5;

use reqwest::r#async::Client; // 0.9.14
use tokio; // 0.1.18

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() {
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let conn = get_connection(db_uri);
    let s3_client = get_s3_client();
    let num_threads = num_cpus::get();
    let mut rows = get_unsplit_filings(&conn);

    let filings_contents = Arc::new(Mutex::new(vec![]));
    let filings_contents2 = filings_contents.clone();

    for _i in 0..num_threads {
        let filings_contents = filings_contents.clone();
        spawn_worker(filings_contents);
    }

    let filing_s3_paths = rows
        .iter()
        .map(|row| {
            let f: Filing = row.into();
            f.filing_edgar_url
        })
        .collect::<Vec<String>>();
    let bodies = stream::iter_ok(filing_s3_paths)
        .map(move |path| {
            let full_path = format!("{}.gz", path);
            let get_req = GetObjectRequest {
                bucket: String::from("birb-edgar-filings"),
                key: full_path,
                ..Default::default()
            };
            s3_client.get_object(get_req).and_then(|result| {
                let stream = result.body.unwrap();
                stream.concat2().from_err()
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    let work = bodies
        .for_each(move |b| {
            println!("Got {} bytes", b.len());
            let filings_contents = filings_contents.clone();
            std::thread::spawn(move || {
                let mut locked = filings_contents.lock().unwrap();
                locked.push(b.to_vec());
            });
            Ok(())
        })
        .map_err(|e| panic!("Error while processing: {}", e));

    tokio::run(work);

    println!(
        "Total items: {}",
        filings_contents2.clone().lock().unwrap().len()
    );
}

fn spawn_worker(filing_contents: Arc<Mutex<Vec<Vec<u8>>>>) {
    let filing_contents = filing_contents.clone();
    thread::spawn(move || loop {
        let mut locked = filing_contents.lock().expect("Oh no!");
        if locked.len() > 0 {
            let first = locked.swap_remove(0);
            println!("Processing item with length: {}", first.len());
        }
    });
}

fn get_unsplit_filings(conn: &Connection) -> Rows {
    let query = format!(
        "
        SELECT * FROM filing f TABLESAMPLE SYSTEM ((100000 * 100) / 5100000.0)
        LEFT JOIN split_filing sf on f.id = sf.filing_id
        WHERE f.collected = true
        AND sf.filing_id IS NULL
        ORDER BY random()
        LIMIT {}
        ;
    ",
        NUMBER_TO_COLLECT_PER_ITERATION
    );
    //    let query = r"select * from filing where filing_edgar_url = 'edgar/data/40545/0000040545-16-000152.txt'";
    // Execute query
    conn.query(&*query, &[]).expect("Couldn't get rows!")
}
