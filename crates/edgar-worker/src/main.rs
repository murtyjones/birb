extern crate api_lib;
extern crate openssl_probe;
use std::env;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use api_lib::models::filer::Model as Filer;
use filer_status_lib::FilerStatus;

use std::error::Error;

use postgres::{Connection, TlsMode};

/// Find a filer with no status and update it.
pub fn main() -> () {
    openssl_probe::init_ssl_cert_env_vars();
    // Only make requests to the SEC every 5 seconds for now,
    // to be on the safe side.
    const SECONDS_DELAY: u16 = 5;

    // Create channels for sending and receieving
    let (one_tx, one_rx) = channel();

    // Spawn one second timer
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(SECONDS_DELAY.into()));
        one_tx.send("next iteration").unwrap();
    });

    loop {
        let _ = one_rx.try_recv().map(|_| {
            update_one_filer();
        });
    }
}

fn update_one_filer() -> () {
    let conn = get_connection();
    let cik = get_cik_for_unset_filer(&conn);
    // Get Latest status for filer
    let filer = Filer { cik };
    let mut filer_status: FilerStatus = FilerStatus::new(filer);
    filer_status.set_is_active();

    // Save result to database
    save_new_filer_status(&conn, &filer_status.1, &filer_status.0.cik);
}

/// Get the database connection
#[cfg(not(test))]
fn get_connection() -> Connection {
    let db_uri = env::var("DATABASE_URI").expect("No connection string found!");
    Connection::connect(db_uri, TlsMode::None).expect("Unable to connect to database!")
}

/// Represents a fake database connection
#[cfg(test)]
struct MockConnection {}

/// Get the mock database connection
#[cfg(test)]
fn get_connection() -> MockConnection {
    MockConnection {}
}

/// Get the CIK of a filer who does not yet have a filing status
#[cfg(not(test))]
fn get_cik_for_unset_filer(conn: &Connection) -> String {
    // Get a random filer with no `active` field set.
    let result = conn.query(
        "SELECT * FROM filer WHERE active IS NULL ORDER BY random() LIMIT 1;",
        &[],
    );
    match result {
        Ok(rows) => {
            println!("{} rows found", rows.len());
            // Panic if # of results != 1
            assert_eq!(rows.len(), 1);
            rows.get(0) // get first (and only) result
                .get(0) // get
        }
        Err(_) => panic!("Can't get filer!"),
    }
}

/// Get a mock CIK of a filer who does not yet have a filing status
#[cfg(test)]
fn get_cik_for_unset_filer(_conn: &MockConnection) -> String {
    String::from("0000000000")
}

/// Update filing status in the database for a given filer
#[cfg(not(test))]
fn save_new_filer_status(conn: &Connection, active: &bool, cik: &String) -> () {
    // TODO: Fix the fact that this execute invocation seems to hang. Not sure why.
    let result = conn.execute(
        "UPDATE filer SET active = $1 WHERE cik = $2",
        &[&active, &cik],
    );
    match result {
        Ok(updated) => {
            println!("{} rows updated with new filer status", updated);
            assert_eq!(updated, 1);
        }
        Err(_) => panic!("Unable to update filer status for {}", cik),
    }
}

/// Mocks the result of an updated filing status - IE, nothing returned
#[cfg(test)]
fn save_new_filer_status(_conn: &MockConnection, _active: &bool, _cik: &String) -> () {
    // Do nothing
    ()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_one_filer() {
        let _r = update_one_filer();
    }

}
