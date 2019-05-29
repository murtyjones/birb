use crate::meta::response::{GetResponse, ObjectTypes};
use crate::models;
use crate::DbConnection;
use rocket_contrib::json::JsonValue;

/// Get a filer by its cik
#[get("/autocomplete/<substr>")]
pub fn get(conn: DbConnection, substr: String) -> JsonValue {
    match models::company::get_typeahead_results(&conn.0, substr) {
        Ok(filer) => json!(GetResponse {
            object_type: ObjectTypes::List,
            has_more: false,
            data: json!(filer),
        }),
        Err(e) => json!({
            "status": "error",
            "error": e.to_string()
        }),
    }
}
