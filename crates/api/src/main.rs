#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate mongodb;
extern crate serde_derive;

use rocket::request::Request;
use rocket_contrib::json::JsonValue;

#[get("/")]
fn index(_db_conn: DbConn) -> &'static str {
    "Hello, from Rust! (With a DB connection!)"
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service not available. Is the DB up?"
}

#[database("mongo_datastore")]
pub struct DbConn(mongodb::db::Database);

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource not found.",
    })
}

fn rocket() -> rocket::Rocket {
    return rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .register(catchers![not_found]);
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    #[test]
    fn bad_get() {
        let client = Client::new(rocket()).unwrap();
        let res = client
            .get("/doesnotexist")
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }

}
