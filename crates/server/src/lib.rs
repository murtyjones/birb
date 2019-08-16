//! Server main thing

#![deny(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate aws;
extern crate dotenv;
extern crate failure;
extern crate serde_json;

#[cfg(debug_assertions)] use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::response::Response;
use rocket::Request;
use std::io::Cursor;

/// Logic for retrieval
pub mod getters;
/// Route handlers
pub mod handlers;
/// Response types
pub mod meta;

// If in test mode, using the test connection string from ROCKET_DATABASES,
// otherwise use `postgres_datastore` from ROCKET_DATABASES
cfg_if! {
    if #[cfg(test)] {
        #[database("testing")]
        pub struct DbConnection(postgres::Connection);
    } else {
        #[database("postgres_datastore")]
        pub struct DbConnection(postgres::Connection);
    }
}

/// Handles CORS things
/// @see: https://github.com/SergioBenitez/Rocket/issues/25#issuecomment-313895086
pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            //            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:9000"));
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}

/// Entrypoint
pub fn launch() {
    rocket().launch();
}

/// Launches the server
fn rocket() -> rocket::Rocket {
    // Load env vars in non-release environments
    #[cfg(debug_assertions)]
    dotenv().expect("Failed to read .env file");

    return rocket::ignite()
        .attach(DbConnection::fairing())
        .attach(CORS())
        .mount(
            "/",
            routes![
                handlers::health_check::get,
                handlers::autocomplete_company::get,
                handlers::company::get_company_filings,
                handlers::company::get_filing_s3_link,
            ],
        )
        .register(catchers![
            handlers::not_found::handler,
            handlers::service_not_available::handler
        ]);
}

/// Test suite
#[cfg(test)]
mod test {}
