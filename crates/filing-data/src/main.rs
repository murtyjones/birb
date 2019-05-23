#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
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

pub fn main() {
    let conn = get_connection();
    let s3_client = get_s3_client();
    let filing_record = get_filing_record(&conn);
    match filing_record {
        Some(record) => {
            println!("Here it is: {:?}", record);
        }
        None => {
            println!("No records left to collect. Have a drink instead.");
        }
    }
    //    let should_process = should_process_for_record();
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

fn get_s3_client() -> S3Client {
    #[cfg(debug_assertions)]
    let mut credentials = ChainProvider::new();
    #[cfg(not(debug_assertions))]
    let mut credentials = InstanceMetadataProvider::new();
    S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        credentials,
        Region::UsEast1,
    )
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
