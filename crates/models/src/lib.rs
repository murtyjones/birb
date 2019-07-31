#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate chrono;

use chrono::prelude::*;


/// Model for a company
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    /// Identifier
    pub short_cik: String,
    /// Company's name
    pub company_name: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct CompanyFilingData {
    /// Identifier
    pub short_cik: String,
    /// Company's name
    pub company_name: String,
    /// Filing's we have for the company
    pub filings: Vec<Filing>,
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct Filing {
    pub id: i32,
    pub short_cik: String,
    pub filing_name: String,
    pub filing_edgar_url: String,
    pub filing_quarter: i32,
    pub filing_year: i32,
    pub collected: bool,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}
