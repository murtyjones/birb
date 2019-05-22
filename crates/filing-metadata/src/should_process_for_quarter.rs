use crate::parse_index::FilingMetadata;
use postgres::{Connection, TlsMode};
use std::env;

use crate::time_periods::Quarter;
use crate::time_periods::Year;

pub fn main(q: Quarter, y: Year) -> Result<ShouldProcess, failure::Error> {
    let conn = get_connection();
    should_process_for_quarter(&conn, &q, &y)
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

pub enum ShouldProcess {
    Yes,
    No,
}

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "index_status")]
pub enum IndexStatus {
    #[postgres(name = "PROCESSED")]
    Processed,
    #[postgres(name = "FAILED")]
    Failed,
}

#[derive(Debug)]
struct Record {
    status: Option<IndexStatus>,
}

fn should_process_for_quarter(
    conn: &Connection,
    q: &Quarter,
    y: &Year,
) -> Result<ShouldProcess, failure::Error> {
    // TODO if provided quarter == current quarter, set status to `null` and return 'Yes'
    let q_as_num = *q as i32;
    let y_as_num = *y as i32;
    let result = conn.query(
        "
            SELECT * FROM edgar_index
            WHERE index_name='master.idx'
            AND index_quarter = $1
            AND index_year = $2
        ",
        &[&q_as_num, &y_as_num],
    );

    match result {
        Ok(r) => {
            assert!(r.len() < 2);
            if 0 == r.len() {
                insert_index_record(conn, q, y);
                return Ok(ShouldProcess::Yes);
            }
            let record = Record {
                status: r.get(0).get("status"),
            };
            match record.status {
                Some(IndexStatus::Processed) => {
                    return Ok(ShouldProcess::No);
                }
                Some(IndexStatus::Failed) => {
                    return Ok(ShouldProcess::Yes);
                }
                None => {
                    return Ok(ShouldProcess::Yes);
                }
            }
        }
        Err(e) => {
            panic!("Couldn't determine whether to process {}Q{}", q, y);
        }
    }
}

fn insert_index_record(conn: &Connection, q: &Quarter, y: &Year) {
    let q_as_num = *q as i32;
    let y_as_num = *y as i32;
    let r = conn.execute(
        "
        INSERT INTO edgar_index
        (index_name, index_quarter, index_year)
        VALUES ('master.idx', $1, $2)
    ",
        &[&q_as_num, &y_as_num],
    );
    match r {
        Ok(records_updated) => {
            assert_eq!(1, records_updated);
        }
        Err(e) => {
            panic!("Couldn't insert record for {}Q{}", q, y);
        }
    }
}
