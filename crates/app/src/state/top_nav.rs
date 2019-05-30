use models::Company;
use serde::{Deserialize, Serialize};

/// Base response object for a GET request
#[derive(Serialize, Deserialize)]
pub struct TopNavSearchBar {
    pub typeahead_results: Option<TypeaheadResponse>,
    pub has_initiated_auto_complete_download: bool,
    pub is_typeahead_open: bool,
    pub typeahead_active_index: Option<i32>,
}

impl TopNavSearchBar {
    pub fn new() -> TopNavSearchBar {
        TopNavSearchBar {
            typeahead_results: None,
            has_initiated_auto_complete_download: false,
            is_typeahead_open: false,
            typeahead_active_index: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TypeaheadResponse {
    pub data: Vec<Company>,
    pub has_more: bool,
    pub object_type: String,
}
