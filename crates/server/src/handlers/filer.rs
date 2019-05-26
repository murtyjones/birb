use crate::meta::response::{GetResponse, ObjectTypes};
use crate::models;
use crate::DbConnection;
use rocket_contrib::json::JsonValue;

/// Get a filer by its cik
#[get("/filer/<cik>")]
pub fn get(conn: DbConnection, cik: String) -> JsonValue {
    match models::filer::find_one_by_cik(&conn.0, cik) {
        Ok(filer) => json!(GetResponse {
            object_type: ObjectTypes::Object,
            has_more: false,
            data: json!(filer),
        }),
        Err(e) => json!({
            "status": "error",
            "error": e.to_string()
        }),
    }
}
