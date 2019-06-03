use crate::getters;
use crate::meta::response::{GetResponse, ObjectTypes};
use crate::DbConnection;
use rocket_contrib::json::JsonValue;

/// Get a company's information
#[get("/companies/<short_cik>/filings")]
pub fn get_filing_info(conn: DbConnection, short_cik: String) -> JsonValue {
    match getters::company::get_filing_info(&conn.0, short_cik) {
        Ok(filing_data) => json!(GetResponse {
            object_type: ObjectTypes::Object,
            has_more: false,
            data: json!(filing_data),
        }),
        Err(e) => json!({
            "status": "error",
            "error": e.to_string()
        }),
    }
}
