use crate::parse_index::FilingMetadata;
use postgres::{Connection, TlsMode};
use std::env;

use crate::should_process_for_quarter::IndexStatus;
use crate::time_periods::Quarter;
use crate::time_periods::Year;

pub fn main(q: Quarter, y: Year, d: Vec<FilingMetadata>) {
    let conn = get_connection();
    persist(&conn, &q, &y, d);
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

fn persist(conn: &Connection, q: &Quarter, y: &Year, d: Vec<FilingMetadata>) -> () {
    let q_as_num = *q as i32;
    let y_as_num = *y as i32;

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
             ON CONFLICT DO NOTHING;
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
             (company_short_cik, filing_name, filing_edgar_url, filing_quarter, filing_year)
             VALUES ($1, $2, $3, $4, $5);
             ",
        )
        .expect("Couldn't prepare filing upsert statement for execution");

    for each in &d {
        filing_upsert_stmt
            .execute(&[
                &each.short_cik,
                &each.form_type,
                &each.filename,
                &q_as_num,
                &y_as_num,
            ])
            .expect("Couldn't execute update");
    }

    trans
        .execute(
            "
            UPDATE edgar_index
            SET status = $1
            WHERE
            index_name = 'master.idx'
            AND index_quarter = $2
            AND index_year = $3
        ",
            &[&IndexStatus::Processed, &q_as_num, &y_as_num],
        )
        .expect("Couldn't update index status");

    trans.commit().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {}
}
