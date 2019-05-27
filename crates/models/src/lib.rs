#[macro_use]
extern crate serde_derive;

/// Model for a company
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    /// Identifier
    pub short_cik: String,
    /// Company's name
    pub company_name: String,
}
