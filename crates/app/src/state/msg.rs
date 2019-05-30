use wasm_bindgen::JsValue;

pub enum Msg {
    SetPath(String),
    /// Deserializes JSON array of typeahead results to `Option<Vec<Company>>`
    SetTypeaheadJson(JsValue),
    /// Represents whether the client is already fetching the typeahead results
    InitiatedTypeaheadRequest,
    /// Represents that the typeahead is open
    TypeaheadOpen(bool),
    /// Represents the key the user pressed
    KeyDown(Option<String>),
}
