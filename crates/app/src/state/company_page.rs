use models::Filing;
use serde::{Deserialize, Serialize};

/// Relates to the top nav
#[derive(Serialize, Deserialize)]
pub struct CompanyPage {
    pub data: Option<Vec<Filing>>,
}

impl CompanyPage {
    pub fn new() -> CompanyPage {
        CompanyPage { data: None }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FilingResponse {
    pub data: Vec<Filing>,
    pub has_more: bool,
    pub object_type: String,
}
