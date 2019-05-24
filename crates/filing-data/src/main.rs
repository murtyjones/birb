#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate reqwest;
use futures::{Future, Stream};
use postgres::{Connection, TlsMode};
use rusoto_core::credential::ChainProvider;
use rusoto_core::credential::InstanceMetadataProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use std::env;
extern crate filing_metadata_lib;

mod filing;
use filing::Filing;
use filing_metadata_lib::download_index::{get_s3_client, store_s3_document};

static BASE_EDGAR_URL: &'static str = "https://www.sec.gov/Archives/";

pub fn main() {
    let conn = get_connection();
    let s3_client = get_s3_client();
    let filing_record = get_filing_record(&conn);
    match filing_record {
        Some(f) => {
            println!("Here it is: {:?}", f);
            let bucket = format!("birb-edgar-filings");
            let file_path = f.filing_edgar_url;
            let document_contents = get_edgar_filing(&file_path).into_bytes();
            println!("Storing doc with file path: {:?}", file_path);
            store_s3_document(&s3_client, &bucket, &file_path, document_contents);
        }
        None => {
            println!("No records left to collect. Have a drink instead.");
        }
    }
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

fn get_filing_record(conn: &Connection) -> Option<Filing> {
    let rows = conn
        .query(
            "
        SELECT * FROM filing
        WHERE collected = false
        ORDER BY random()
        LIMIT 1",
            &[],
        )
        .expect("Couldn't retrieve a random filing!");
    assert!(2 > rows.len(), "Query should have returned 1 or 0 records!");
    if rows.len() == 1 {
        let record = rows.get(0);
        let filing = Filing {
            id: record.get("id"),
            company_short_cik: record.get("company_short_cik"),
            filing_name: record.get("filing_name"),
            filing_edgar_url: record.get("filing_edgar_url"),
            filing_quarter: record.get("filing_quarter"),
            filing_year: record.get("filing_year"),
        };
        return Some(filing);
    }
    None
}

fn get_edgar_filing(file_path: &String) -> String {
    let mut url: String = BASE_EDGAR_URL.to_owned();
    url.push_str(file_path);
    reqwest::get(url.as_str())
        .expect("Couldn't make request")
        .text()
        .expect("Couldn't get text from request")
}
