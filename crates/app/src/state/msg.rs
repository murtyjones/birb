use wasm_bindgen::JsValue;
use web_sys::Element;

pub enum Msg {
    SetPath(String),
    /// Deserializes JSON array of typeahead results to `Option<Vec<Company>>`
    SetTypeaheadJson(JsValue),
    /// Deserializes JSON array of typeahead results to `Option<Vec<Filing>>`
    SetCompanyPageFilings(JsValue),
    /// Represents whether the client is already fetching the typeahead results
    InitiatedTypeaheadRequest,
    /// Represents that the typeahead is open
    TypeaheadOpen(bool),
    /// Represents the key the user pressed
    KeyDown(Option<String>),
    /// Represents that the user has clicked
    Click(Option<Element>),
    /// Represents whether the side nav can be seen by the user
    SetSideNavVisibility(bool),
}
