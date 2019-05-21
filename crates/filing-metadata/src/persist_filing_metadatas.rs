use crate::parse_index::FilingMetadata;
use postgres::{Connection, TlsMode};
use std::env;

use crate::time_periods::Quarter;
use crate::time_periods::Year;

pub fn main(q: Quarter, y: Year, d: Vec<FilingMetadata>) {
    let conn = get_connection();
    persist_companies(&conn, &q, &y, d);
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

fn persist_companies(conn: &Connection, q: &Quarter, y: &Year, d: Vec<FilingMetadata>) -> () {
    let q_as_num = *q as i32;
    let y_as_num = *y as i32;

    let trans = conn.transaction().expect("Couldn't begin transaction");

    let stmt = trans
        .prepare(
            "
             INSERT INTO company
             (short_cik, company_name)
             VALUES ($1, $2)
             ON CONFLICT (short_cik) DO NOTHING;
             ",
        )
        .expect("Couldn't prepare statement for execution");
    for each in d {
        stmt.execute(&[&each.short_cik, &each.company_name])
            .expect("Couldn't execute update");
    }

    trans.commit().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {}
}
