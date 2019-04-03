use bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetResponse {
    pub _id: ObjectId,
    pub CIK: String,
}
