extern crate filing_metadata;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

use strum::IntoEnumIterator;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use postgres::{Connection, TlsMode};

use crate::filing_metadata::time_periods::{Quarter,Year};
use crate::filing_metadata::parse_index;
use postgres::transaction::Transaction;
use filing_metadata::parse_index::FilingMetadata;
use postgres::stmt::Statement;

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
    let mut file = std::fs::File::open(path_to_index_file).expect(&format!("No index file found for: {}Q{}", q, y));
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}

pub fn get_prod_conn_string() -> String {
    let port = std::str::from_utf8(include_bytes!("../../../../scripts/local_port")).unwrap().to_string();
    let uname = std::str::from_utf8(include_bytes!("../../../../terraform/out/rds_db_username")).unwrap().to_string();
    let passwd = std::str::from_utf8(include_bytes!("../../../../terraform/out/rds_db_password")).unwrap().to_string();
    let db_name = std::str::from_utf8(include_bytes!("../../../../terraform/out/rds_db_name")).unwrap().to_string();

    format!("postgres://{}:{}@localhost:{}/{}", uname, passwd, port, db_name)
}

/// Get the database connection
fn get_connection() -> Connection {
//    Connection::connect(get_prod_conn_string(), TlsMode::None).expect("Unable to connect to database!")
    Connection::connect("postgres://postgres:develop@localhost:5432/postgres", TlsMode::None)
        .expect("Unable to connect to database!")
}

fn perform(y: Year, q: Quarter) {
    let conn = get_connection();
    let contents = get_index_contents(y,q);
    let filing_metadatas = parse_index::main(contents).expect("Couldn't parse index!");
    let trans = conn.transaction().expect("Couldn't begin transaction");
    let stmt = trans
        .prepare(
            "
             INSERT INTO filing
             (company_short_cik, filing_name, filing_edgar_url, filing_quarter, filing_year, collected, date_filed)
             VALUES ($1, $2, $3, $4, $5, false, $6)
             ON CONFLICT (filing_edgar_url) DO
                UPDATE SET date_filed = $6;
         ",
        )
        .expect("Couldn't prepare company upsert statement for execution");
    for f in filing_metadatas.iter() {
        let q_as_num = *&q as i32;
        let y_as_num = *&y as i32;
        stmt.execute(&[
            &f.short_cik, &f.form_type, &f.filename, &q_as_num, &y_as_num, &f.date_filed,
        ]).expect("Couldn't execute update");
    }
    trans.commit().unwrap()
}

fn main() {
    for year in Year::iter() {
        for quarter in Quarter::iter() {
            perform(year, quarter);
        }
    }
}
