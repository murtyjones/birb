/// Response object for filer
#[derive(Debug, Deserialize, Serialize)]
pub struct GetResponse {
    /// the central index key
    pub cik: String,
    /// the names array
    pub names: Vec<String>,
}
