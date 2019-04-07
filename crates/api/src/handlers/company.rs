use bson;

use crate::meta;
use crate::models;
use crate::DbConnection;
use rocket_contrib::json::{Json, JsonValue};

#[get("/company/<cik>")]
pub fn get(conn: DbConnection, cik: String) -> Json<JsonValue> {
    let doc = models::company::Model::find_one_by_cik(&conn.0, cik.to_owned());
    //    let _company = bson::Bson::Document(doc);
    let company = bson::from_bson::<meta::company::GetResponse>(bson::Bson::Document(doc));
    Json(json!({
        "status": "ok",
        "data": company.unwrap()
    }))
}
