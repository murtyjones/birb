use aws::s3::{get_s3_client, store_s3_document_gzipped_async};
use chrono::Utc;
use failure;
use filing_parser::split_full_submission::split_full_submission;
use futures::{future, stream, Future, Stream};
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::rows::{Row, Rows};
use postgres::Connection;
use rayon::prelude::*;
use rusoto_core::{ByteStream, RusotoFuture};
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use rusoto_s3::{GetObjectError, PutObjectOutput};
use rusoto_s3::{GetObjectOutput, GetObjectRequest};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio_core::reactor::Core;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection};

const NUMBER_TO_COLLECT_PER_ITERATION: i32 = 20;
const PARALLEL_REQUESTS: usize = 5;

use futures::future::FromErr;
use futures::stream::Concat2;
use tokio;

fn main() {
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let conn = get_connection(db_uri);
    let s3_client = get_s3_client();
    let num_threads = num_cpus::get();
    let rows = get_unsplit_filings(&conn);
    let filings = rows
        .into_iter()
        .map(|row| row.into())
        .collect::<Vec<Filing>>();

    let filings_to_process_queue = Arc::new(Mutex::new(vec![]));
    let filings_to_process_queue2 = filings_to_process_queue.clone();

    for _i in 0..num_threads {
        let filings_to_process_queue = filings_to_process_queue.clone();
        spawn_worker(filings_to_process_queue);
    }

    let bodies = stream::iter_ok(filings)
        .map(move |filing| {
            let full_path = format!("{}.gz", filing.filing_edgar_url);
            let get_req = GetObjectRequest {
                bucket: String::from("birb-edgar-filings"),
                key: full_path,
                ..Default::default()
            };
            s3_client.get_object(get_req).and_then(move |result| {
                let stream: FromErr<Concat2<ByteStream>, std::io::Error> =
                    result.body.unwrap().concat2().from_err();
                std::boxed::Box::new(future::ok((stream, filing)))
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    let work = bodies
        .for_each(move |(result, filing)| {
            let body = result.wait().unwrap().to_vec();
            let filings_to_process_queue = filings_to_process_queue.clone();
            let mut locked = filings_to_process_queue.lock().unwrap();
            locked.push((body, filing));
            Ok(())
        })
        .map_err(|e| panic!("Error while processing: {}", e));

    tokio::run(work);

    println!("Took: {:?}", start.elapsed());
}

fn spawn_worker(queue: Arc<Mutex<Vec<(Vec<u8>, Filing)>>>) {
    let queue = queue.clone();
    thread::spawn(move || loop {
        let first = {
            let mut locked = queue.lock().expect("Oh no!");
            locked.pop()
        };
        if let Some((contents, filing)) = first {
            println!("Processing item: {:?}", filing.id);
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
