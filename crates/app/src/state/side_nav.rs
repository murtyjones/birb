use serde::{Deserialize, Serialize};

/// Sop nav
#[derive(Serialize, Deserialize)]
pub struct SideNav {
    pub is_visible: bool,
}

impl SideNav {
    pub fn new() -> SideNav {
        SideNav { is_visible: true }
    }
}
