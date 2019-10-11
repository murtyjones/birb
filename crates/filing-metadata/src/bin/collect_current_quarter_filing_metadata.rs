extern crate chrono;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate filing_metadata;
extern crate postgres;
extern crate reqwest;
extern crate utils;
use crate::chrono::Datelike;

use filing_metadata::parse_index;
use filing_metadata::parse_index::FilingMetadata;
use filing_metadata::time_periods::Quarter;
use filing_metadata::time_periods::Year;
use postgres::Connection;
use utils::get_connection;

fn main() {
    env_logger::init();

    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let conn = get_connection(db_uri);
    let current_qtr = get_current_quarter();
    let current_year = get_current_year();
    let current_qtr = 4;
    let current_year = 2019;
    let url = format!(
        "https://www.sec.gov/Archives/edgar/full-index/{}/QTR{}/master.idx",
        current_year, current_qtr
    );

    info!("Getting document...");
    let contents = reqwest::get(url.as_str())
        .expect("Couldn't make request")
        .text()
        .expect("Couldn't get text from request");

    info!("Document retrieved. Parsing...");

    let filing_metadatas: Vec<FilingMetadata> =
        parse_index::main(contents).expect("Unable to parse index file");

    info!("Document parsed. Persisting...");
    persist(&conn, current_qtr, current_year, filing_metadatas);

    info!("All done!");
    // download index from sec.gov
    // parse it
    // sql transaction: insert into filings ignore conflicts
    // NOTE: don't forget logging

    // once built:
    // run for Q1, Q2 and Q3 2019 to ensure they are up to date
    // build a lambda that runs this every hour
}

fn get_current_year() -> i32 {
    let now = chrono::Utc::now();
    now.year()
}

fn get_current_quarter() -> i32 {
    let month = chrono::Utc::now().month();
    match month {
        1|2|3 => 1,
        4|5|6 => 2,
        7|8|9 => 3,
        _ /* IE. 10|11|12*/ => 4,
    }
}

fn persist(conn: &Connection, qtr: i32, year: i32, d: Vec<FilingMetadata>) -> () {
    let trans = conn.transaction().expect("Couldn't begin transaction");

    let company_upsert_stmt = trans
        .prepare(
            "
             INSERT INTO company
             (short_cik, company_name)
             VALUES ($1, $2)
             ON CONFLICT (short_cik) DO NOTHING;
             ",
        )
        .expect("Couldn't prepare company upsert statement for execution");

    for each in &d {
        company_upsert_stmt
            .execute(&[&each.short_cik, &each.company_name])
            .expect("Couldn't execute update");
    }

    let filing_type_upsert_stmt = trans
        .prepare(
            "
             INSERT INTO filing_type
             (filing_name)
             VALUES ($1)
             ON CONFLICT DO NOTHING
             ",
        )
        .expect("Couldn't prepare filing type upsert statement for execution");

    for each in &d {
        filing_type_upsert_stmt
            .execute(&[&each.form_type])
            .expect("Couldn't execute update");
    }

    let filing_upsert_stmt = trans
        .prepare(
            "
             INSERT INTO filing
             (company_short_cik, filing_name, filing_edgar_url, date_filed, filing_quarter, filing_year)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT DO NOTHING
             ",
        )
        .expect("Couldn't prepare filing upsert statement for execution");

    for each in &d {
        filing_upsert_stmt
            .execute(&[
                &each.short_cik,
                &each.form_type,
                &each.filename,
                &each.date_filed,
                &qtr,
                &year,
            ])
            .expect("Couldn't execute update");
    }

    trans.commit().unwrap()
}
