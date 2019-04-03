#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate mongodb;
extern crate serde_json;

mod handlers;
mod meta;
mod models;

#[database("mongo_datastore")]
pub struct DbConn(mongodb::db::Database);

#[catch(503)]
fn service_not_available(_req: &rocket::request::Request) -> &'static str {
    "Service not available. Is the DB up?"
}

#[catch(404)]
fn not_found() -> rocket_contrib::json::JsonValue {
    json!({
        "status": "error",
        "reason": "Resource not found.",
    })
}

fn rocket() -> rocket::Rocket {
    return rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![handlers::company::get])
        .register(catchers![not_found, service_not_available]);
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
