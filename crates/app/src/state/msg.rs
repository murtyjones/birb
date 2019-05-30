use wasm_bindgen::JsValue;

pub enum Msg {
    Click,
    SetPath(String),
    /// Deserializes JSON array of Github contributors to `Option<Vec<PercyContributor>>`
    SetContributorsJson(JsValue),
    /// Deserializes JSON array of Github contributors to `Option<Vec<Company>>`
    SetTypeaheadJson(JsValue),
    /// Represents whether the client is already fetching the JSON array of Github contributors
    InitiatedContributorsDownload,
    /// Represents whether the client is already fetching the typeahead results
    InitiatedTypeaheadRequest,
    /// Represents that the typeahead is open
    TypeaheadOpen(bool),
}
