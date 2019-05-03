#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
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

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(do_filer_status_update);

    Ok(())
}

#[cfg(not(test))]
fn get_connection() -> Connection {
    Connection::connect(
        env::var("DATABASE_URI").unwrap(), TlsMode::None
    ).unwrap()
}

#[cfg(test)]
struct MockConnection {}

#[cfg(test)]
fn get_connection() -> MockConnection {
    MockConnection {}
}

#[cfg(not(test))]
fn get_cik_for_unset_filer(conn: &Connection) -> String {
    // Get filer to update
    let cik = conn
        .query("SELECT * FROM filer WHERE active IS NULL LIMIT 1", &[])
        .unwrap()
        .get(0)
        .get(0);

    cik
}

#[cfg(test)]
fn get_cik_for_unset_filer(_conn: &MockConnection) -> String {
    String::from("0000000000")
}

#[cfg(not(test))]
fn save_new_filer_status(conn: &Connection, active: &bool, cik: &String) -> () {
    let result = conn.execute(
        "UPDATE filer SET active = $1 WHERE cik = $2",
        &[&active, &cik]
    );
    match result {
        Ok(_) => (),
        Err(_) => panic!("Unable to update filer status for {}", cik),
    }
}

#[cfg(test)]
fn save_new_filer_status(conn: &MockConnection, active: &bool, cik: &String) -> () {
    // Do nothing
    ()
}

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
