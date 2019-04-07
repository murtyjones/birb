//use bson::Bson::Boolean;
use rocket_contrib::json::JsonValue;

#[derive(Serialize, Deserialize)]
pub enum ResponseStatuses {
    OK,
    ERR,
}

#[derive(Serialize, Deserialize)]
pub enum ObjectTypes {
    Object,
    List,
}

#[derive(Serialize, Deserialize)]
pub struct GetResponse {
    pub status: ResponseStatuses,
    pub object_type: ObjectTypes,
    pub has_more: bool,
    pub data: JsonValue,
}
