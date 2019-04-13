use crate::mongo::Mongo;
use bson;
use bson::Array;
use mongodb::db::Database;

use mongodb::ordered::OrderedDocument;

/// Model for a filer
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// A Central Index Key or CIK number is a number given to an individual, company, or foreign
    /// government by the United States Securities and Exchange Commission. The number is used to
    /// identify its filings in several online databases, including EDGAR.
    pub cik: String,
    /// Names used by the entity. There can be multiple
    pub names: Array,
}

impl Model {
    /// Find an entity using its cik
    pub fn find_one_by_cik(conn: &Database, cik: String) -> OrderedDocument {
        Mongo::find_one(conn, "filer", Some(doc! { "cik" => cik })).unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn find_one_by_cik_unwraps() {}
}
