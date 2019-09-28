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
pub fn get_split_filing(conn: DbConnection, _short_cik: String, filing_id: i32) -> JsonValue {
    match getters::company::get_split_filing(&conn.0, filing_id) {
        Ok(docs) => json!(GetResponse {
            object_type: ObjectTypes::List,
            has_more: false,
            data: json!(docs),
        }),
        Err(e) => json!({
            "status": "error",
            "error": e.to_string()
        }),
    }
}
