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

fn _do_filer_status_update() -> FilerStatus{
    let conn = Connection::connect(env::var("DATABASE_URI").unwrap(), TlsMode::None).unwrap();

    // Get filer to update
    let cik = conn
        .query("SELECT * FROM filer WHERE active IS NULL LIMIT 1", &[])
        .unwrap()
        .get(0)
        .get(0);

    // Get Latest status for filer
    let filer = Filer { cik };
    let mut filer_status: FilerStatus = FilerStatus::new(filer);
    filer_status.set_is_active();

    // Save result to database
    let update_result = conn
        .execute("UPDATE filer SET active = $1 WHERE cik = $2", &[&filer_status.1, &filer_status.0.cik])
        .unwrap();

    filer_status
}

fn do_filer_status_update(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    let filer_status = _do_filer_status_update();
    Ok(CustomOutput {
        message: format!("Set active status for cik {} to '{}'", &filer_status.0.cik, &filer_status.1),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_do_filer_status_update() {
        __do_filer_status_update();
    }

}
