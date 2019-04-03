use bson;
use rocket_contrib::json::JsonValue;

use crate::DbConn;

use crate::models;

#[get("/company/<cik>")]
pub fn get(conn: DbConn, cik: String) -> JsonValue {
    let doc = models::company::Model::find_one_by_cik(&conn, cik.to_owned()).unwrap();
    let company = bson::Bson::Document(doc);
    json!({
        "status": "ok",
        "data": company
    })
}
