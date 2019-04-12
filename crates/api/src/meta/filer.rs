use bson::oid::ObjectId;
use bson::Array;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetResponse {
    pub _id: ObjectId,
    pub cik: String,
    pub names: Array,
}
