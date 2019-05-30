use models::Company;
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::Cell;
use std::rc::Rc;

mod msg;
pub use self::msg::Msg;

#[derive(Serialize, Deserialize)]
pub struct State {
    click_count: Rc<Cell<u32>>,
    path: String,
    contributors: Option<Vec<PercyContributor>>,
    typeahead_results: Option<TypeaheadResponse>,
    has_initiated_contributors_download: bool,
    has_initiated_auto_complete_download: bool,
    is_typeahead_open: bool,
    key_down: Option<String>,
}

impl State {
    pub fn new(count: u32) -> State {
        State {
            path: "/".to_string(),
            click_count: Rc::new(Cell::new(count)),
            contributors: None,
            typeahead_results: None,
            has_initiated_contributors_download: false,
            has_initiated_auto_complete_download: false,
            is_typeahead_open: false,
            key_down: None,
        }
    }

    pub fn from_json(state_json: &str) -> State {
        serde_json::from_str(state_json).unwrap()
    }
}

impl State {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl State {
    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::Click => self.increment_click(),
            Msg::SetPath(path) => self.set_path(path.to_string()),
            Msg::SetContributorsJson(json) => {
                self.contributors = Some(json.into_serde().unwrap());
            }
            Msg::SetTypeaheadJson(json) => {
                self.typeahead_results = Some(json.into_serde().unwrap());
            }
            Msg::InitiatedContributorsDownload => {
                self.has_initiated_contributors_download = true;
            }
            Msg::InitiatedTypeaheadRequest => {
                self.has_initiated_auto_complete_download = true;
            }
            Msg::TypeaheadOpen(v) => {
                self.is_typeahead_open = *v;
            }
            Msg::KeyDown(v) => match v {
                Some(key) => {
                    self.key_down = Some(key.clone());
                }
                None => {
                    self.key_down = None;
                }
            },
        };
    }

    pub fn click_count(&self) -> u32 {
        self.click_count.get()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn contributors(&self) -> &Option<Vec<PercyContributor>> {
        &self.contributors
    }

    pub fn typeahead_results(&self) -> &Option<TypeaheadResponse> {
        &self.typeahead_results
    }

    pub fn has_initiated_contributors_download(&self) -> &bool {
        &self.has_initiated_contributors_download
    }

    pub fn has_initiated_auto_complete_download(&self) -> &bool {
        &self.has_initiated_auto_complete_download
    }

    pub fn is_typeahead_open(&self) -> &bool {
        &self.is_typeahead_open
    }

    pub fn key_down(&self) -> &Option<String> {
        &self.key_down
    }
}

impl State {
    fn increment_click(&mut self) {
        self.click_count.set(self.click_count.get() + 1);
    }

    fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

// Serde ignores fields not in this struct when deserializing
#[derive(Serialize, Deserialize)]
pub struct PercyContributor {
    /// Github username.
    pub login: String,
    /// Github profile URL. E.g. https://github.com/username
    pub html_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct TypeaheadResponse {
    pub data: Vec<Company>,
    pub has_more: bool,
    pub object_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        let state_json = r#"{"click_count":5,"path":"/","contributors":null,"has_initiated_contributors_download":false,"has_initiated_auto_complete_download":false,}"#;

        let state = State::from_json(state_json);

        assert_eq!(state.click_count(), 5);

        assert_eq!(&state.to_json(), state_json);
    }
}
