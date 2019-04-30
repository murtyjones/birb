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
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    let conn = Connection::connect("postgres://postgres@localhost:5433", TlsMode::None).unwrap();
    Ok(CustomOutput {
        message: format!("Hello!"),
    })
}
