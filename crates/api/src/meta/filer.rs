use bson::oid::ObjectId;
use bson::Array;

/// Response object for filer
#[derive(Debug, Deserialize, Serialize)]
pub struct GetResponse {
    /// the _id assigned by the DB to the object
    pub _id: ObjectId,
    /// the central index key
    pub cik: String,
    /// the names array
    pub names: Array,
}
