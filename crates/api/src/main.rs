#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate bson;
extern crate mongodb;
use mongodb::db::ThreadedDatabase;
use mongodb::ordered::OrderedDocument;
use rocket::request::Request;
use rocket_contrib::json::JsonValue;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rust! (With a DB connection!)"
}

#[database("mongo_datastore")]
struct DbConn(mongodb::db::Database);

fn find_company_by_cik(conn: &mongodb::db::Database, cik: String) -> Option<OrderedDocument> {
    conn.collection("company")
        .find_one(Some(doc! { "CIK" => cik }), None)
        .unwrap()
}

#[get("/company/<cik>")]
fn get_company(conn: DbConn, cik: String) -> JsonValue {
    let doc = find_company_by_cik(&conn, cik).unwrap();
    let company = bson::Bson::Document(doc);
    json!({
        "status": "ok",
        "doc": company
    })
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
        .mount("/", routes![index, get_company])
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
