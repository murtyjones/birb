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

/// Gets a signed URL for the sequence == 1 document of a filing
#[get("/companies/<short_cik>/filings/<filing_id>/raw-s3-link")]
pub fn get_signed_url(conn: DbConnection, short_cik: i32, filing_id: i32) -> JsonValue {
    match getters::company::get_signed_url(&conn.0, short_cik, filing_id) {
        Ok(signed_url) => json!(GetResponse {
            object_type: ObjectTypes::Object,
            has_more: false,
            data: json!(signed_url),
        }),
        Err(e) => json!({
            "status": "error",
            "error": e.to_string()
        }),
    }
}
