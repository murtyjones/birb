extern crate filing_metadata;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

use postgres::{Connection, TlsMode};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

use crate::filing_metadata::parse_index;
use crate::filing_metadata::time_periods::{Quarter, Year};
use filing_metadata::parse_index::FilingMetadata;
use postgres::stmt::Statement;
use postgres::transaction::Transaction;

lazy_static! {
    /// The Birb source code root directory.
    ///
    /// For different developers this will be a different folder on the file system.
    ///
    /// Since we don't know where that folder is, but we do know that it is two directories
    /// above the birb-cli directory, we just use a relative path and canonicalize it
    ///  (remove the dots, basically)
    pub static ref BB_ROOT_DIR: PathBuf = {
        let bb_root_dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/../..";
        let bb_root_dir = PathBuf::from(bb_root_dir);
        let bb_root_dir = bb_root_dir.canonicalize().unwrap();
        bb_root_dir
    };
}

/// Get the path to the root directory of your Birb app repo.
///
/// ex: /Users/sally/code/birb/birb-project
pub fn bb_root_dir() -> &'static Path {
    BB_ROOT_DIR.as_ref()
}

fn get_index_contents(y: Year, q: Quarter) -> String {
    let root = bb_root_dir().to_string_lossy();
    let path_to_index_file = format!("{}/data/edgar-indexes/{}/QTR{}/master.idx", root, y, q);
    let mut file = std::fs::File::open(path_to_index_file)
        .expect(&format!("No index file found for: {}Q{}", q, y));
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}

pub fn get_prod_conn_string() -> String {
    let port = format!("{}/../../scripts/local_port", env!("CARGO_MANIFEST_DIR"));
    let port = std::path::Path::new(&port);
    let username = format!(
        "{}/../../scripts/terraform/out/rds_db_username",
        env!("CARGO_MANIFEST_DIR")
    );
    let username = std::path::Path::new(&username);
    let password = format!(
        "{}/../../scripts/terraform/out/rds_db_password",
        env!("CARGO_MANIFEST_DIR")
    );
    let password = std::path::Path::new(&password);
    let db_name = format!(
        "{}/../../scripts/terraform/out/rds_db_name",
        env!("CARGO_MANIFEST_DIR")
    );
    let db_name = std::path::Path::new(&db_name);

    if username.is_file() && password.is_file() && db_name.is_file() {
        let port = std::fs::read_to_string(port).unwrap();
        let username = std::fs::read_to_string(username).unwrap();
        let password = std::fs::read_to_string(password).unwrap();
        let db_name = std::fs::read_to_string(db_name).unwrap();
        return format!(
            "postgres://{}:{}@localhost:{}/{}",
            username, password, port, db_name
        );
    }
    panic!("No prod conn string available!")
}

/// Get the database connection
fn get_connection() -> Connection {
    Connection::connect(get_prod_conn_string(), TlsMode::None)
        .expect("Unable to connect to database!")
    //    Connection::connect(
    //        "postgres://postgres:develop@localhost:5432/postgres",
    //        TlsMode::None,
    //    )
    //    .expect("Unable to connect to database!")
}

fn get_current_year() -> i32 {
    let now = Utc::now();
    now.year()
}

fn get_current_quarter() -> i32 {
    let month = Utc::now().month();
    match month {
        1|2|3 => 1,
        4|5|6 => 2,
        7|8|9 => 3,
        _ /*10|11|12*/ => 4,
    }
}

fn past_current_quarter(y: Year, q: Quarter) -> bool {
    let q_as_num = *&q as i32;
    let y_as_num = *&y as i32;
    let current_q = get_current_quarter();
    let current_y = get_current_year();
    // Past current quarter in current year
    if q_as_num > current_q && y_as_num == current_y {
        return true;
    }
    // Past current year
    return y_as_num > current_y;
}

fn perform(y: Year, q: Quarter) {
    if past_current_quarter(y, q) {
        return ();
    };
    let conn = get_connection();
    let contents = get_index_contents(y, q);
    let filing_metadatas = parse_index::main(contents).expect("Couldn't parse index!");
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

    let filing_upsert_stmt = trans
        .prepare(
            "
             INSERT INTO filing
             (company_short_cik, filing_name, filing_edgar_url, filing_quarter, filing_year, collected, date_filed)
             VALUES ($1, $2, $3, $4, $5, false, $6)
             ON CONFLICT (filing_edgar_url) DO
                UPDATE SET date_filed = $6;
         ",
        )
        .expect("Couldn't prepare filing upsert statement for execution");
    for f in filing_metadatas.iter() {
        let q_as_num = *&q as i32;
        let y_as_num = *&y as i32;
        println!("Doing for {}", f.filename);

        company_upsert_stmt
            .execute(&[&f.short_cik, &f.company_name])
            .expect("Couldn't execute company upsert");

        filing_upsert_stmt
            .execute(&[
                &f.short_cik,
                &f.form_type,
                &f.filename,
                &q_as_num,
                &y_as_num,
                &f.date_filed,
            ])
            .expect("Couldn't execute filing metadata upsert");
    }
    trans.commit().unwrap()
}

use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    // Make a vector to hold the children which are spawned.
    //    let mut children = vec![];
    //
    //    children.push(std::thread::spawn(move || {
    perform(Year::TwentySeventeen, Quarter::Three);
    //    }));

    //    children.push(std::thread::spawn(move || {
    //        perform(Year::TwentySeventeen, Quarter::Two);
    //    }));
    //
    //    children.push(std::thread::spawn(move || {
    //        perform(Year::TwentySeventeen, Quarter::Three);
    //    }));
    //
    //    children.push(std::thread::spawn(move || {
    //        perform(Year::TwentySeventeen, Quarter::Four);
    //    }));
    //
    //    for child in children {
    //        // Wait for the thread to finish. Returns a result.
    //        let _ = child.join();
    //    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    //    for year in Year::iter() {
    //        for quarter in Quarter::iter() {
    //            perform(year, quarter);
    //        }
    //    }
}
