extern crate aws;
extern crate chrono;
extern crate failure;
extern crate models;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate reqwest;
extern crate rusoto_core;

use aws::s3::{get_s3_client, store_s3_document_gzipped};
use chrono::prelude::*;
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

use utils::{compress_gzip, get_accession_number, get_cik, get_connection_pool};

use futures::future::FromErr;
use futures::stream::Concat2;
use r2d2::PooledConnection;
use tokio;

const PARALLEL_REQUESTS: usize = 3;
static BASE_EDGAR_URL: &'static str = "https://www.sec.gov/Archives/";

fn main() {
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let pool = get_connection_pool(db_uri);
    let s3_client = get_s3_client();
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
            let utc: DateTime<Utc> = Utc::now();
            let body = body.wait().unwrap().to_vec();
            println!("Collected at: {}", utc);
            let pool = pool.clone();
            let conn = pool.get().unwrap();
            let r = store_s3_document_gzipped(
                &s3_client,
                "birb-edgar-filings",
                &*filing.filing_edgar_url,
                body,
                "private",
            );
            match r {
                Ok(()) => {
                    persist_collected_filing_status_to_db(conn, filing);
                }
                Err(e) => println!("Error storing document: {}", e),
            }
            Ok(())
        })
        .map_err(|e| println!("Error while downloading: {}", e));

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

fn persist_collected_filing_status_to_db(
    conn: PooledConnection<PostgresConnectionManager>,
    filing: Filing,
) {
    let query = "
        UPDATE filing SET collected = true WHERE id = $1
    ";
    let result = conn.query(&*query, &[&filing.id]);
    match result {
        Ok(_) => {
            println!("Uploaded: {}", filing.filing_edgar_url);
        }
        Err(e) => {
            println!("Error updating DB for collected filing! {:?}", e);
        }
    }
}
