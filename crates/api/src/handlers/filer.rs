use bson;

use crate::meta;
use crate::meta::response::{GetResponse, ObjectTypes, ResponseStatuses};
use crate::models;
use crate::DbConnection;
use rocket_contrib::json::{Json, JsonValue};

#[get("/filer/<cik>")]
pub fn get(conn: DbConnection, cik: String) -> Json<JsonValue> {
    let doc = models::filer::Model::find_one_by_cik(&conn.0, cik.to_owned());
    let filer = bson::from_bson::<meta::filer::GetResponse>(bson::Bson::Document(doc));
    Json(json!(GetResponse {
        status: ResponseStatuses::OK,
        object_type: ObjectTypes::Object,
        has_more: false,
        data: json!(filer.unwrap()),
    }))
}