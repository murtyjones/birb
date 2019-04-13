//! API main thing

#![deny(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate mongodb;
extern crate serde_json;

#[cfg(debug_assertions)] use dotenv::dotenv;

/// Route handlers
pub mod handlers;
/// Response types
pub mod meta;
/// DB models
pub mod models;
/// DB module
pub mod mongo;

/// Struct to handle the DB connection
#[database("mongo_datastore")]
pub struct DbConnection(mongodb::db::Database);

/// Launches the server
fn rocket() -> rocket::Rocket {
    // Load env vars in non-release environments
    #[cfg(debug_assertions)]
    dotenv().ok();

    return rocket::ignite()
        .attach(DbConnection::fairing())
        .mount(
            "/",
            routes![handlers::health_check::get, handlers::filer::get],
        )
        .register(catchers![
            handlers::not_found::handler,
            handlers::service_not_available::handler
        ]);
}

/// Entrypoint
pub fn launch() {
    rocket().launch();
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
        assert!(body.contains("TESLA MOTORS INC"));
        assert!(body.contains("0001318605"));
    }

}
