use bson;

use crate::meta;
use crate::meta::response::{GetResponse, ObjectTypes, ResponseStatuses};
use crate::models;
use crate::DbConnection;
use rocket_contrib::json::{Json, JsonValue};

#[get("/company/<cik>")]
pub fn get(conn: DbConnection, cik: String) -> Json<JsonValue> {
    dbg!("get from db");
    let doc = models::company::Model::find_one_by_cik(&conn.0, cik.to_owned());
    dbg!("resp from db");
    dbg!(&doc);
    let company = bson::from_bson::<meta::company::GetResponse>(bson::Bson::Document(doc));
    dbg!("converted");
    dbg!(&company);
    Json(json!(GetResponse {
        status: ResponseStatuses::OK,
        object_type: ObjectTypes::Object,
        has_more: false,
        data: json!(company.unwrap()),
    }))
}
