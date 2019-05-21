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

enum ShouldProcess {
    Yes,
    No,
}

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "index_status")]
enum IndexStatus {
    #[postgres(name = "PROCESSED")]
    Processed,
    #[postgres(name = "FAILED")]
    Failed,
}

#[derive(Debug)]
struct Record {
    status: Option<IndexStatus>,
}

fn do_query(conn: &Connection, q: &Quarter, y: &Year) -> Result<ShouldProcess, failure::Error> {
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
            println!("Records: {}", r.len());
            assert!(r.len() < 2);
            if 0 == r.len() {
                return Ok(ShouldProcess::Yes);
            }
            let record = Record {
                status: r.get(0).get("status"),
            };
            println!("{:?}", record);
            match record.status {
                Some(IndexStatus::Processed) => {}
                Some(IndexStatus::Failed) => {}
                None => {}
            }
            Ok(ShouldProcess::Yes)
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
