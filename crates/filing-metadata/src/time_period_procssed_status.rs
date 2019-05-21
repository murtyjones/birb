use crate::parse_index::FilingMetadata;
use postgres::{Connection, TlsMode};
use std::env;

use crate::time_periods::Quarter;
use crate::time_periods::Year;

pub fn main(q: Quarter, y: Year) {
    let conn = get_connection();
    let query = do_query(&conn, &q, &y);
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

fn do_query(conn: &Connection, q: &Quarter, y: &Year) {
    let q_as_num = *q as i32;
    let y_as_num = *y as i32;
    let result = conn.query(
        "
            SELECT * FROM edgar_indexes
            WHERE index_name='master.idx'
            AND index_quarter=$1
            AND index_year=$2
        ",
        &[&q_as_num, &y_as_num],
    );

    match result {
        Ok(r) => {
            assert!(r.len() >= 0);
            assert!(r.len() < 2);
            println!("Records: {}", r.len());
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    //    for row in  {
    //        let status: String = row.get(3);
    //        println!("{:?}", status);
    //    }
}
