use crate::getters;
use crate::meta::response::{GetResponse, ObjectTypes};
use crate::DbConnection;
use rocket_contrib::json::JsonValue;

/// Get a company's information
#[get("/companies/<short_cik>/filings")]
pub fn get_company_filings(conn: DbConnection, short_cik: String) -> JsonValue {
    match getters::company::get_company_filings(&conn.0, short_cik) {
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

/// Get a company's information
#[get("/companies/<_short_cik>/filings/<filing_id>/raw-s3-link")]
pub fn get_filing_s3_link(conn: DbConnection, _short_cik: String, filing_id: i32) -> JsonValue {
    match getters::company::get_filing_s3_link(&conn.0, filing_id) {
        Ok(filing_link) => json!(GetResponse {
            object_type: ObjectTypes::Object,
            has_more: false,
            data: json!(filing_link),
        }),
        Err(e) => json!({
            "status": "error",
            "error": e.to_string()
        }),
    }
}
