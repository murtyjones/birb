#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate mongodb;
extern crate serde_derive;

use mongodb::db::ThreadedDatabase;
use mongodb::Bson;
use rocket::request::Request;
use rocket_contrib::json::JsonValue;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rust! (With a DB connection!)"
}

#[database("mongo_datastore")]
struct DbConn(mongodb::db::Database);

fn use_connection(conn: &mongodb::db::Database) -> () {
    let coll = ThreadedDatabase::collection(conn, "company");
    let cursor = coll.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref cik)) = item.get("CIK") {
                println!("cik: {}", cik);
            }
        }
    }
}

#[get("/co")]
fn use_company_collection(conn: DbConn) -> () {
    // this parameter doesn't feel right...
    use_connection(&conn.0)
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service not available. Is the DB up?"
}

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
        .mount("/", routes![index, use_company_collection])
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
