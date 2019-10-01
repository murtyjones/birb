#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate chrono;

use chrono::prelude::*;
use postgres::rows::{Row, Rows};

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

impl<'a> From<Row<'a>> for Filing {
    fn from(row: Row) -> Self {
        Filing {
            id: row.get("id"),
            company_short_cik: row.get("company_short_cik"),
            filing_name: row.get("filing_name"),
            filing_edgar_url: row.get("filing_edgar_url"),
            filing_quarter: row.get("filing_quarter"),
            filing_year: row.get("filing_year"),
            collected: row.get("collected"),
            date_filed: row.get("date_filed"),
            created_at: Some(row.get("created_at")),
            updated_at: Some(row.get("updated_at")),
        }
    }
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
    pub sequence: i32,
    pub filename: String,
    pub description: Option<String>,
    pub doc_type: String,
    pub s3_url_prefix: String,
    pub split_filing_created_at: Option<chrono::DateTime<Utc>>,
    pub split_filing_updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct SplitDocumentBeforeUpload {
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
    /// TODO: This shouldn't be persisted to the DB... so how does this struct need to change?
    pub text: String,
    /// If the text is uuencoded, this is the decoded text
    pub decoded_text: Option<Vec<u8>>,
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
    /// Where this lives in S3. Should be in
    /// the format of: `edgar/data/111111/1111111111-11-111111`
    pub s3_url_prefix: String,
    /// When it was saved to our DB.
    pub created_at: Option<chrono::DateTime<Utc>>,
    /// When it was last updated in our DB.
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl SplitDocument {
    pub fn from_rows(rows: &Rows) -> Vec<SplitDocument> {
        rows.iter()
            .map(|row| SplitDocument {
                filing_id: row.get("filing_id"),
                doc_type: row.get("doc_type"),
                sequence: row.get("sequence"),
                filename: row.get("filename"),
                description: row.get("description"),
                s3_url_prefix: row.get("s3_url_prefix"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect::<Vec<SplitDocument>>()
    }
}

impl<'a> From<Row<'a>> for SplitDocument {
    fn from(row: Row) -> Self {
        SplitDocument {
            filing_id: row.get("filing_id"),
            doc_type: row.get("doc_type"),
            sequence: row.get("sequence"),
            filename: row.get("filename"),
            description: row.get("description"),
            s3_url_prefix: row.get("s3_url_prefix"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct SignedUrl {
    pub short_cik: i32,
    pub filing_id: i32,
    pub signed_url: String,
}
