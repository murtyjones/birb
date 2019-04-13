use bson;
use bson::Array;
use mongodb::db::Database;

use mongodb::db::ThreadedDatabase;

const FILER_COLLECTION: &str = "filer";

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

/// Find an entity using its cik
pub fn find_one_by_cik(conn: &Database, cik: String) -> Result<Model, &str> {
    let filter = Some(doc! { "cik" => cik });
    match conn
        .collection(FILER_COLLECTION)
        .find_one(filter, None)
        .expect("fail")
    {
        Some(result) => Ok(Model {
            // not loving this:
            cik: result.get("cik").unwrap().as_str().unwrap().to_string(),
            names: result.get("names").unwrap().as_array().unwrap().to_vec(),
        }),
        None => Err("Not found"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn find_one_by_cik_unwraps() {}
}
