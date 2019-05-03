#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate more_asserts;
extern crate simple_logger;
extern crate api_lib;
use std::env;

use api_lib::models::filer::Model as Filer;
use filer_status_lib::FilerStatus;
use lambda::error::HandlerError;

use std::error::Error;

use postgres::{Connection, TlsMode};

#[derive(Deserialize, Clone)]
struct CustomEvent {}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

/// Entrypoint for the lambda
fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(do_filer_status_update);

    Ok(())
}

/// Find a filer with no status and update it.
fn do_filer_status_update(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    let conn = get_connection();
    let cik = get_cik_for_unset_filer(&conn);
    // Get Latest status for filer
    let filer = Filer { cik };
    let mut filer_status: FilerStatus = FilerStatus::new(filer);
    filer_status.set_is_active();

    // Save result to database
    save_new_filer_status(&conn, &filer_status.1, &filer_status.0.cik);
    Ok(CustomOutput {
        message: format!("Set active status for cik {} to '{}'", &filer_status.0.cik, &filer_status.1),
    })
}

#[cfg(not(test))]
/// Get the database connection
fn get_connection() -> Connection {
    Connection::connect(
        env::var("DATABASE_URI").unwrap(), TlsMode::None
    ).unwrap()
}

#[cfg(test)]
/// Represents a fake database connection
struct MockConnection {}

#[cfg(test)]
/// Get the mock database connection
fn get_connection() -> MockConnection {
    MockConnection {}
}

#[cfg(not(test))]
/// Get the CIK of a filer who does not yet have a filing status
fn get_cik_for_unset_filer(conn: &Connection) -> String {
    // Get filer to update
    let result = conn
        .query("SELECT * FROM filer WHERE active IS NULL LIMIT 1", &[]);
    match result {
        Ok(rows) => {
            println!("{} rows found", rows.len());
            // Should either be 0 or 1:
            assert_gt!(rows.len(), 0);
            assert_lt!(rows.len(), 1);
            rows
                .get(0) // get first (and only) result
                .get(0) // get
        },
        Err(_) => panic!("Can't get filer!"),
    }
}

#[cfg(test)]
/// Get a mock CIK of a filer who does not yet have a filing status
fn get_cik_for_unset_filer(_conn: &MockConnection) -> String {
    String::from("0000000000")
}

#[cfg(not(test))]
/// Update filing status in the database for a given filer
fn save_new_filer_status(conn: &Connection, active: &bool, cik: &String) -> () {
    // TODO: Fix the fact that this execute invocation seems to hang. Not sure why.
    let result = conn.execute(
        "UPDATE filer SET active = $1 WHERE cik = $2",
        &[&active, &cik]
    );
    match result {
        Ok(updated) => {
            println!("{} rows updated with new filer status", updated);
            assert_eq!(updated, 1);
        },
        Err(_) => panic!("Unable to update filer status for {}", cik),
    }
}

#[cfg(test)]
/// Mocks the result of an updated filing status - IE, nothing returned
fn save_new_filer_status(conn: &MockConnection, active: &bool, cik: &String) -> () {
    // Do nothing
    ()
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_do_filer_status_update() {
        let mock_context = lambda::Context {
            aws_request_id: String::from("mock"),
            client_context: None,
            deadline: 10,
            function_name: String::from("mock"),
            function_version: String::from("mock"),
            identity: None,
            invoked_function_arn: String::from("mock"),
            log_group_name: String::from("mock"),
            log_stream_name: String::from("mock"),
            memory_limit_in_mb: 128,
            xray_trace_id: None,
        };
        let r = do_filer_status_update(CustomEvent {}, mock_context);
    }

}
