use aws::s3::{get_s3_client, store_s3_document_gzipped_async};
use failure;
use filing_parser::split_full_submission::{split_full_submission, SplittingError};
use futures::{future, stream, Future, Stream};
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::rows::Rows;
use postgres::Connection;
use r2d2_postgres::PostgresConnectionManager;
use reqwest::r#async::{Client, Decoder};
use rusoto_core::ByteStream;
use rusoto_s3::GetObjectRequest;
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio_core::reactor::Core;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection_pool};

use futures::future::FromErr;
use futures::stream::Concat2;
use r2d2;
use r2d2::PooledConnection;
use tokio;

const PARALLEL_REQUESTS: usize = 5;
static BASE_EDGAR_URL: &'static str = "https://www.sec.gov/Archives/";

fn main() {
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let pool = get_connection_pool(db_uri);
    let client = Client::new();
    let filings_to_collect: Vec<Filing> = get_not_yet_collected_filings(pool.get().unwrap())
        .into_iter()
        .map(|row| row.into())
        .collect::<Vec<Filing>>();

    let bodies = stream::iter_ok(filings_to_collect)
        .map(move |filing| {
            let url = format!("{}{}", BASE_EDGAR_URL, filing.filing_edgar_url);
            client.get(&*url).send().and_then(|res| {
                let stream: FromErr<Concat2<Decoder>, reqwest::Error> =
                    res.into_body().concat2().from_err();
                std::boxed::Box::new(future::ok((stream, filing)))
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    let work = bodies
        .for_each(move |(body, filing)| {
            let s = decompress_gzip(body.wait().unwrap().to_vec());
            Ok(())
        })
        .map_err(|e| panic!("Error while processing: {}", e));

    tokio::run(work);
}

/// Get filings that need to be collected
fn get_not_yet_collected_filings(conn: PooledConnection<PostgresConnectionManager>) -> Rows {
    let query = "
        SELECT * FROM filing TABLESAMPLE SYSTEM ((100000 * 100) / 5100000.0)
        WHERE collected IS NOT true
        AND filing_year = 2019
        AND filing_quarter = 2
        ;
    ";
    conn.query(&*query, &[])
        .expect("Couldn't update filing status to collected = false!")
}
