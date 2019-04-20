use postgres::Connection;

const FILER_TABLE: &str = "filer";

/// Model for a filer
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// A Central Index Key or CIK number is a number given to an individual, company, or foreign
    /// government by the United States Securities and Exchange Commission. The number is used to
    /// identify its filings in several online databases, including EDGAR.
    pub cik: String,
    /// Names used by the entity. There can be multiple
    pub names: Vec<String>,
}

/// Find an entity using its cik
pub fn find_one_by_cik(conn: &Connection, cik: String) -> Result<Model, &str> {
    let query = format!(
        "SELECT * FROM {table} WHERE CIK={cik}",
        table = FILER_TABLE,
        cik = cik
    );
    let results = conn.query(&*query, &[]).unwrap();
    Ok(Model {
        cik: results.get(0).get(0),
        names: results.get(0).get(1),
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn find_one_by_cik_unwraps() {}
}
