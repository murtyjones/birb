use rocket_contrib::json::JsonValue;

/// What type of object is being returned to the: a json object or array
#[derive(Serialize, Deserialize)]
pub enum ObjectTypes {
    /// JSON object e.g. { my_thing: 'val' }
    Object,
    /// JSON array e.g. [ 'hello!', 'world' ]
    List,
}

/// Base response object for a GET request
#[derive(Serialize, Deserialize)]
pub struct GetResponse {
    /// Which type of object is the data
    pub object_type: ObjectTypes,
    /// Are the more pages to this response?
    pub has_more: bool,
    /// The actual json data
    pub data: JsonValue,
}
