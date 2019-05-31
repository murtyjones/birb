use csv;
use failure;
use postgres::{Connection, TlsMode};
use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::vec::Vec;
use std::{env, io, process};

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
pub struct CikToTicker {
    pub short_cik: String,
    pub ticker: String,
    pub name: String,
    pub exchange: String,
    pub sic: String,
    pub business: String,
    pub incorporated: String,
    pub irs: String,
}

pub fn main() -> Result<(), failure::Error> {
    let content = get_cik_ticker_mapping_csv();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_reader(content.as_bytes());
    let mut mappings = vec![];
    for (i, result) in rdr.deserialize().enumerate() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: CikToTicker = result?;
        mappings.push(record);
    }
    upsert_mappings(mappings);
    Ok(())
}

const CIK_TO_TICKER_URL: &'static str = "http://rankandfiled.com/static/export/cik_ticker.csv";

fn get_cik_ticker_mapping_csv() -> String {
    reqwest::get(CIK_TO_TICKER_URL)
        .expect("Couldn't make request")
        .text()
        .expect("Couldn't get text from request")
}

fn upsert_mappings(mappings: Vec<CikToTicker>) -> Result<(), failure::Error> {
    let conn = get_connection();
    let trans = conn.transaction().expect("Couldn't begin transaction");

    // TODO figure out how to ignore insert errors when a short_cik foreign key isnt found
    let upsert_stmt = trans
        .prepare(
            "
             INSERT INTO ticker
             (ticker, exchange, company_short_cik)
             VALUES ($1, $2, $3);
             ",
        )
        .expect("Couldn't prepare company upsert statement for execution");

    for each in &mappings {
        upsert_stmt
            .execute(&[&each.ticker, &each.exchange, &each.short_cik])
            .expect("Couldn't execute update");
    }

    trans.commit().unwrap();

    Ok(())
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}
