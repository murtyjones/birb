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

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyFilingData {
    /// Identifier
    pub short_cik: String,
    /// Company's name
    pub company_name: String,
    /// Filings we have for the company
    pub filings: Vec<Filing>,
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
/// Used for the `filing` table
pub struct Filing {
    pub id: i32,
    pub company_short_cik: String,
    pub filing_name: String,
    pub filing_edgar_url: String,
    pub filing_quarter: i32,
    pub filing_year: i32,
    pub collected: bool,
    pub date_filed: chrono::NaiveDate,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
/// Represents the `filing` table join with the `company` table
pub struct CompanyFilingJoined {
    /// Identifier
    pub short_cik: String,
    /// Company's name
    pub company_name: String,
    pub company_created_at: Option<chrono::DateTime<Utc>>,
    pub company_updated_at: Option<chrono::DateTime<Utc>>,
    pub id: i32,
    pub filing_name: String,
    pub filing_edgar_url: String,
    pub filing_quarter: i32,
    pub filing_year: i32,
    pub collected: bool,
    pub date_filed: chrono::NaiveDate,
    pub filing_created_at: Option<chrono::DateTime<Utc>>,
    pub filing_updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct SplitDocument {
    /// Foreign key - the filing that this split document relates to.
    /// This is a one-to-many relationship from the `filing` to the
    /// `split_filing` table.
    pub filing_id: i32,
    /// contents of the <TYPE> node. Ex. "GRAPHIC", "10-Q", "EX-101.2"
    /// `filing_type` table even though it might seem like it should
    pub doc_type: String,
    /// The sequence of the document for display purposes,
    /// beginning at `1`, which is the most important
    pub sequence: i32,
    /// The filename of the document (e.g. "d490575d10q.htm")
    /// from the <FILENAME> node
    pub filename: String,
    /// The SEC's description of the document from
    /// the <DESCRIPTION> node (e.g. "FORM 10-Q")
    pub description: Option<String>,
    /// The actual document contents from the <TEXT> node
    pub text: String,
    /// When it was saved to our DB.
    pub created_at: Option<chrono::DateTime<Utc>>,
    /// When it was last updated in our DB.
    pub updated_at: Option<chrono::DateTime<Utc>>,
}
