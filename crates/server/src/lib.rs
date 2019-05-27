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
extern crate dotenv;
extern crate postgres;
extern crate serde_json;

use app::App;
#[cfg(debug_assertions)] use dotenv::dotenv;
use rocket::http::ContentType;
use rocket::response::Response;
use rocket_contrib::serve::StaticFiles;
use std::io::Cursor;

/// Route handlers
pub mod handlers;
/// Response types
pub mod meta;
/// DB models
pub mod models;

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
        .mount("/", routes![index, favicon, catch_all])
        .mount("/static", StaticFiles::from(static_files.as_str()))
        .mount(
            "/api",
            routes![handlers::health_check::get, handlers::filer::get],
        )
        .register(catchers![
            handlers::not_found::handler,
            handlers::service_not_available::handler
        ]);
}

/// # Example
///
/// localhost:7878/?init=50
#[get("/?<initial_count>")]
fn index(initial_count: Option<u32>) -> Result<Response<'static>, ()> {
    respond("/".to_string(), initial_count)
}

/// # Example
///
/// localhost:7878/contributors?init=1200
#[get("/<path>?<initial_count>")]
fn catch_all(path: String, initial_count: Option<u32>) -> Result<Response<'static>, ()> {
    respond(path, initial_count)
}

/// Favicon
#[get("/favicon.ico")]
fn favicon() -> &'static str {
    ""
}

/// Responder
fn respond(path: String, initial_count: Option<u32>) -> Result<Response<'static>, ()> {
    let app = App::new(initial_count.unwrap_or(1000), path);
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
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    /// Health check should return OK
    #[test]
    fn good_health_check() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let res = client.get("/").header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
    }

    /// nonexistent route should Not Found
    #[test]
    fn bad_get() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let res = client
            .get("/doesnotexist")
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }

    /// Get a filer successfully
    #[test]
    fn get_tsla() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut res = client
            .get("/filer/0001318605")
            .header(ContentType::JSON)
            .dispatch();
        let body = res.body_string().unwrap();
        assert_eq!(res.status(), Status::Ok);
        assert!(body.contains("0001318605"));
    }

}
