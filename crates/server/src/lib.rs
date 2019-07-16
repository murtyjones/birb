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
extern crate dotenv;
extern crate failure;
extern crate serde_json;

use app::App;
#[cfg(debug_assertions)] use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::response::Response;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
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

/// html to be replaced
const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
/// initial state to be replaced
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

/// base index file
static INDEX_HTML: &str = include_str!("../index.html");

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

    let static_files = format!("{}/../client/build", env!("CARGO_MANIFEST_DIR"));

    return rocket::ignite()
        .attach(DbConnection::fairing())
        .attach(CORS())
        .mount("/", routes![index, favicon, company, catch_all])
        .mount("/static", StaticFiles::from(static_files.as_str()))
        .mount(
            "/api",
            routes![
                handlers::health_check::get,
                handlers::autocomplete_company::get,
                handlers::company::get_filing_info
            ],
        )
        .register(catchers![
            handlers::not_found::handler,
            handlers::service_not_available::handler
        ]);
}

/// # Example
///
/// localhost:7878/
#[get("/")]
fn index() -> Result<Response<'static>, ()> {
    respond("/".to_string())
}

/// # Example
///
/// localhost:7878/thing
#[get("/companies/<short_cik>")]
fn company(short_cik: String) -> Result<Response<'static>, ()> {
    respond("/companies/".to_string() + &*short_cik)
}

/// # Example
///
/// localhost:7878/thing
#[get("/<path>")]
fn catch_all(path: String) -> Result<Response<'static>, ()> {
    respond(path)
}

/// Favicon
#[get("/favicon.ico")]
fn favicon() -> &'static str {
    ""
}

/// Responder
fn respond(path: String) -> Result<Response<'static>, ()> {
    let app = App::new(path);
    let state = app.store.borrow();

    let html = format!("{}", include_str!("../index.html"));
    let html = html.replacen(HTML_PLACEHOLDER, &app.render().to_string(), 1);
    let html = html.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);

    let mut response = Response::new();
    response.set_header(ContentType::HTML);
    response.set_sized_body(Cursor::new(html));

    Ok(response)
}

/// Test suite
#[cfg(test)]
mod test {}
