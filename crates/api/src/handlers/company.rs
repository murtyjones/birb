use bson;
use rocket_contrib::json::JsonValue;

use crate::meta;
use crate::models;
use crate::DbConn;

#[get("/company/<cik>")]
pub fn get(conn: DbConn, cik: String) -> JsonValue {
    let doc = models::company::Model::find_one_by_cik(&conn, cik.to_owned());
    //    let _company = bson::Bson::Document(doc);
    let company = bson::from_bson::<meta::company::GetResponse>(bson::Bson::Document(doc.unwrap()));
    json!({
        "status": "ok",
        "data": company
    })
}
