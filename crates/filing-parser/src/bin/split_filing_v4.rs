#[macro_use]
extern crate log;
use aws::s3::{get_s3_client, store_s3_document_gzipped_async};
use failure;
use filing_parser::split_full_submission::{split_full_submission, SplittingError};
use futures::{future, stream, Future, Stream};
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::rows::Rows;
use postgres::Connection;
use r2d2_postgres::PostgresConnectionManager;
use rusoto_core::ByteStream;
use rusoto_s3::GetObjectRequest;
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio_core::reactor::Core;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection_pool};

use env_logger;
use futures::future::FromErr;
use futures::stream::Concat2;
use r2d2;
use r2d2::PooledConnection;
use tokio;

const PARALLEL_REQUESTS: usize = 5;
const MIN_QUEUE_SIZE: usize = 10;

fn main() {
    env_logger::init();
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let pool = get_connection_pool(db_uri);
    let num_threads = num_cpus::get();
    let filings = Arc::new(Mutex::new(
        get_unsplit_filings(pool.get().unwrap())
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<Filing>>(),
    ));

    let mut handles = vec![];

    let queue = Arc::new(Mutex::new(vec![]));

    handles.push(spawn_requester(queue.clone(), filings.clone()));

    for _i in 0..num_threads {
        let queue = queue.clone();
        let pool = pool.clone();
        handles.push(spawn_worker(queue, pool.get().unwrap(), filings.clone()));
    }

    // wait for all threads to resolve
    for h in handles {
        h.join().unwrap();
    }

    info!("Took: {:?}", start.elapsed());
}

fn spawn_requester(
    queue: Arc<Mutex<Vec<(String, Filing)>>>,
    filings: Arc<Mutex<Vec<Filing>>>,
) -> JoinHandle<()> {
    info!("Requester");
    thread::spawn(move || loop {
        let queue = queue.clone();
        let filings = filings.clone();
        let (queue_size, filings_left) = {
            (
                queue.clone().lock().unwrap().len(),
                filings.clone().lock().unwrap().len(),
            )
        };

        if queue_size + filings_left == 0 {
            break;
        } else if queue_size < MIN_QUEUE_SIZE && filings_left > 0 {
            info!("Queue at {}, adding more filings.", queue_size);
            let mut filings_to_download = vec![];
            for _i in 0..10 {
                if let Some(f) = filings.lock().unwrap().pop() {
                    filings_to_download.push(f);
                }
            }
            let bodies = stream::iter_ok(filings_to_download)
                .map(move |filing| {
                    let full_path = format!("{}.gz", filing.filing_edgar_url);
                    let get_req = GetObjectRequest {
                        bucket: String::from("birb-edgar-filings"),
                        key: full_path,
                        ..Default::default()
                    };
                    let s3_client = get_s3_client();
                    s3_client.get_object(get_req).and_then(move |result| {
                        info!("Got object");
                        let stream: FromErr<Concat2<ByteStream>, std::io::Error> =
                            result.body.unwrap().concat2().from_err();
                        std::boxed::Box::new(future::ok((stream, filing)))
                    })
                })
                .buffer_unordered(PARALLEL_REQUESTS);

            let work = bodies
                .for_each(move |(result, filing)| {
                    let body = decompress_gzip(result.wait().unwrap().to_vec());
                    let queue = queue.clone();
                    let mut filings_to_process_queue = queue.lock().unwrap();
                    info!("Adding to work queue");
                    filings_to_process_queue.push((body, filing));
                    Ok(())
                })
                .map_err(|e| panic!("Error while processing: {}", e));

            tokio::run(work);
        }
    })
}

fn spawn_worker(
    queue: Arc<Mutex<Vec<(String, Filing)>>>,
    conn: PooledConnection<PostgresConnectionManager>,
    filings: Arc<Mutex<Vec<Filing>>>,
) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let maybe_filing_to_process = {
            let queue = queue.clone();
            let maybe_doc = queue.lock().unwrap().pop();
            maybe_doc
        };
        if let Some((object_contents, filing)) = maybe_filing_to_process {
            info!("Processing one");
            let split_filings = split_full_submission(&*object_contents, &filing.id);
            match split_filings {
                Ok(docs) => {
                    let cik = get_cik(&*filing.filing_edgar_url);
                    let accession_number = get_accession_number(&*filing.filing_edgar_url);
                    let s3_url_prefix = format!("edgar/data/{}/{}", cik, accession_number);
                    let s3_client = get_s3_client();
                    upload_all(&s3_client, &filing, &docs).expect("Couldn't persist to DB!");
                    persist_split_filing_to_db(&conn, &docs, &s3_url_prefix, &filing.id);
                }
                Err(e) => match e {
                    SplittingError::WrongWebPageHasBeenCollected { .. } => {
                        reset_filing_to_not_collected(&conn, &filing.id);
                    }
                },
            }
        } else {
            if filings.lock().unwrap().len() == 0 {
                info!("Nothing left...");
                // Nothing left for the thread to work on
                break;
            }
        }
    })
}

fn upload_all(
    s3_client: &S3Client,
    filing: &Filing,
    split_filings: &Vec<SplitDocumentBeforeUpload>,
) -> Result<(), failure::Error> {
    info!("Uploading...");
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
        Ok(_items) => info!("Uploaded!"),
        Err(e) => panic!("Error completing futures: {}", e),
    };

    Ok(())
}

fn persist_split_filing_to_db(
    conn: &Connection,
    split_filings: &Vec<SplitDocumentBeforeUpload>,
    s3_url_prefix: &String,
    filing_id: &i32,
) {
    info!("Persisting");
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

    for each in split_filings {
        statement
            .execute(&[
                filing_id,
                &each.sequence,
                &each.doc_type,
                &each.filename,
                &each.description,
                s3_url_prefix,
            ])
            .expect("Couldn't execute update");
    }

    trans.commit().expect("Couldn't insert into split_filing")
}

fn get_unsplit_filings(conn: PooledConnection<PostgresConnectionManager>) -> Rows {
    let query = format!(
        "
        SELECT * FROM filing f TABLESAMPLE SYSTEM ((100000 * 100) / 5100000.0)
        LEFT JOIN split_filing sf on f.id = sf.filing_id
        WHERE f.collected = true
        AND sf.filing_id IS NULL
        ORDER BY random()
        ;
    "
    );
    //    let query = r"select * from filing where filing_edgar_url = 'edgar/data/40545/0000040545-16-000152.txt'";
    // Execute query
    conn.query(&*query, &[]).expect("Couldn't get rows!")
}

fn reset_filing_to_not_collected(conn: &Connection, filing_id: &i32) {
    let query = format!(
        "UPDATE filing SET collected = false WHERE id = {};",
        filing_id
    );
    conn.query(&*query, &[filing_id])
        .expect("Couldn't update filing status to collected = false!");
}
