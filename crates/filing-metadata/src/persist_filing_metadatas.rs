use crate::parse_index::FilingMetadata;
use postgres::{Connection, TlsMode};
use std::env;

use crate::time_periods::Quarter;
use crate::time_periods::Year;

pub fn main(q: Quarter, y: Year, d: Vec<FilingMetadata>) {
    let conn = get_connection();
    let query = build_query(q, y, d);
}

/// Get the database connection
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

fn build_query(q: Quarter, y: Year, d: Vec<FilingMetadata>) {
    let mut query = String::from("BEGIN;\n");
    // TODO build tx query from filing metadata
    query.push_str("COMMIT;\n");
}

fn persist(conn: &Connection, query: &String) -> () {
    let result = conn.execute(query, &[]);
    match result {
        Ok(updated) => {
            println!("{} rows updated with new filer status", updated);
            assert_eq!(updated, 1);
        }
        Err(_) => panic!("Unable to persist"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {}
}
