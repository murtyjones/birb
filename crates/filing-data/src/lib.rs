#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
#[macro_use]
extern crate log;
extern crate aws;
extern crate env_logger;
extern crate filing_metadata;
extern crate reqwest;

use futures::{Future, Stream};
use postgres::{Connection, TlsMode};
use std::env;

mod filing;
use aws::s3::get_s3_client;
use aws::s3::store_s3_document_gzipped;
use filing::Filing;

static BASE_EDGAR_URL: &'static str = "https://www.sec.gov/Archives/";

pub fn get_one_filing() {
    let conn = get_connection();
    let s3_client = get_s3_client();
    let filing_record = get_filing_record(&conn);
    match filing_record {
        Some(f) => {
            info!("Here it is: {:?}", f.id);
            let bucket = format!("birb-edgar-filings");
            let file_path = &f.filing_edgar_url;
            let document_contents = get_edgar_filing(file_path).into_bytes();
            info!("Storing doc with file path: {:?}", file_path);
            store_s3_document_gzipped(
                &s3_client,
                &bucket,
                &file_path,
                document_contents,
                "private",
            )
            .expect("Couldn't store filing in S3");
            info!("Updating status for collected to 'true'");
            persist_document_storage_status(&conn, &f);
            info!("Done!");
        }
        None => {
            info!("No records left to collect. Have a drink instead.");
        }
    }
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

/// Used to get one randomly selected, not-yet collected filing
/// from the 2019 collection of filings.
const RANDOM_FILING_QUERY: &'static str = r#"
    SELECT * FROM filing
    WHERE collected = false
    AND filing_year = 2019
    ORDER by random()
    LIMIT 1;
"#;

fn get_filing_record(conn: &Connection) -> Option<Filing> {
    let rows = conn
        .query(RANDOM_FILING_QUERY, &[])
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
        let collected: bool = record.get("collected");
        assert_eq!(false, collected, "Filing should not already be collected!");
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

fn persist_document_storage_status(conn: &Connection, filing: &Filing) {
    let r = conn
        .execute(
            "
        UPDATE filing
        SET collected = true
        WHERE id = $1

    ",
            &[&filing.id],
        )
        .expect("Couldn't perform update");
    assert_eq!(r, 1, "Expected one record to to be updated!");
}
