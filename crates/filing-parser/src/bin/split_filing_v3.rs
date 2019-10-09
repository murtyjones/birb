use aws::s3::{get_s3_client, store_s3_document_gzipped_async};
use chrono::Utc;
use failure;
use filing_parser::split_full_submission::split_full_submission;
use futures::{future, stream, Future, Stream};
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::rows::{Row, Rows};
use postgres::Connection;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use rayon::prelude::*;
use rusoto_core::{ByteStream, RusotoFuture};
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use rusoto_s3::{GetObjectError, PutObjectOutput};
use rusoto_s3::{GetObjectOutput, GetObjectRequest};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio_core::reactor::Core;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection};

const NUMBER_TO_COLLECT_PER_ITERATION: i32 = 20;
const PARALLEL_REQUESTS: usize = 5;
const MINIMUM_QUEUE_SIZE: usize = 20;

use futures::future::FromErr;
use futures::stream::Concat2;
use r2d2;
use r2d2::{Pool, PooledConnection};
use tokio;

fn main() {
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let manager = PostgresConnectionManager::new(db_uri, TlsMode::None).unwrap();
    let pool = Pool::new(manager).unwrap();
    let s3_client = get_s3_client();
    let num_threads = num_cpus::get();
    let rows = get_unsplit_filings(pool.get().unwrap());
    let filings = rows
        .into_iter()
        .map(|row| row.into())
        .collect::<Vec<Filing>>();

    println!("Got {}", filings.len());

    let filings_to_process_queue = Arc::new(Mutex::new(vec![]));

    let mut handles = vec![];
    for _i in 0..num_threads {
        let pool = pool.clone();
        let filings_to_process_queue = filings_to_process_queue.clone();
        handles.push(spawn_worker(pool, filings_to_process_queue));
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
            println!("Adding to work queue");
            locked.push((body, filing));
            Ok(())
        })
        .map_err(|e| panic!("Error while processing: {}", e));

    tokio::run(work);

    // wait for all threads to resolve
    for h in handles {
        h.join().unwrap();
    }

    println!("Took: {:?}", start.elapsed());
}

fn spawn_worker(
    pool: Pool<PostgresConnectionManager>,
    queue: Arc<Mutex<Vec<(Vec<u8>, Filing)>>>,
) -> JoinHandle<()> {
    let queue = queue.clone();
    thread::spawn(move || loop {
        let pool = pool.clone();
        let (first, len) = {
            let mut locked = queue.lock().expect("Oh no!");
            (locked.pop(), locked.len())
        };
        if let Some((contents, filing)) = first {
            println!("Processing item: {:?}", filing.id);
            let decompressed = decompress_gzip(contents);
            let docs =
                split_full_submission(&*decompressed, &filing.id).expect("couldnt split file");
            upload_all(&filing, &docs).expect("Couldn't upload docs");
            let conn = pool.get().unwrap();
            persist_split_filings_to_db(conn, &filing, &docs).expect("Couldn't persist filings");
            if len == 0 {
                println!("Nothing left to process! Exiting");
                break;
            }
        }
        //        if len < MINIMUM_QUEUE_SIZE {
        //            println!(
        //                "Queue length is {}, below the threshold of {}. Adding another item",
        //                len, MINIMUM_QUEUE_SIZE
        //            );
        //        }
    })
}

fn get_unsplit_filings(conn: PooledConnection<PostgresConnectionManager>) -> Rows {
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

fn upload_all(
    filing: &Filing,
    split_filings: &Vec<SplitDocumentBeforeUpload>,
) -> Result<(), failure::Error> {
    let s3_client = get_s3_client();
    let mut core = Core::new().unwrap();

    let promises = future::join_all(split_filings.iter().map(|doc| {
        let cik = get_cik(&*filing.filing_edgar_url);
        let accession_number = get_accession_number(&*filing.filing_edgar_url);
        let s3_url_prefix = format!("edgar/data/{}/{}", cik, accession_number);
        let contents_for_file = match &doc.decoded_text {
            Some(d) => d.clone(),
            None => doc.text.clone().into_bytes(),
        };
        let path = format!("{}/{}", s3_url_prefix, doc.filename);
        // The first document is always the important one, and therefore the
        // one that we want to restrict read access.
        let acl = match doc.sequence {
            1 => "private",
            _ => "public-read",
        };
        store_s3_document_gzipped_async(
            &s3_client,
            "birb-edgar-filings",
            &*path,
            contents_for_file,
            acl,
        )
    }));

    match core.run(promises) {
        Ok(_items) => println!("Uploaded!"),
        Err(e) => panic!("Error completing futures: {}", e),
    };

    Ok(())
}

fn persist_split_filings_to_db(
    conn: PooledConnection<PostgresConnectionManager>,
    filing: &Filing,
    split_filings: &Vec<SplitDocumentBeforeUpload>,
) -> Result<(), failure::Error> {
    let trans = conn.transaction().expect("Couldn't begin transaction");
    let statement = trans
        .prepare(
            "
             INSERT INTO split_filing
             (filing_id, sequence, doc_type, filename, description, s3_url_prefix)
             VALUES ($1, $2, $3, $4, $5, $6);
             ",
        )
        .expect("Couldn't prepare company upsert statement for execution");

    for doc in split_filings {
        let cik = get_cik(&*filing.filing_edgar_url);
        let accession_number = get_accession_number(&*filing.filing_edgar_url);
        let s3_url_prefix = format!("edgar/data/{}/{}", cik, accession_number);
        statement
            .execute(&[
                &filing.id,
                &doc.sequence,
                &doc.doc_type,
                &doc.filename,
                &doc.description,
                &s3_url_prefix,
            ])
            .expect("Couldn't execute update");
    }
    trans.commit().expect("Couldn't insert into split_filing");
    Ok(())
}
