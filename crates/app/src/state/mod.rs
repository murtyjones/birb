use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::Cell;
use std::rc::Rc;

mod top_nav;
use top_nav::TopNavSearchBar;

mod msg;
pub use self::msg::Msg;
use crate::state::top_nav::TypeaheadResponse;

#[derive(Serialize, Deserialize)]
pub struct State {
    click_count: Rc<Cell<u32>>,
    path: String,
    contributors: Option<Vec<PercyContributor>>,
    has_initiated_contributors_download: bool,
    top_nav_search_bar: TopNavSearchBar,
}

impl State {
    pub fn new(count: u32) -> State {
        State {
            path: "/".to_string(),
            click_count: Rc::new(Cell::new(count)),
            contributors: None,
            has_initiated_contributors_download: false,
            top_nav_search_bar: TopNavSearchBar::new(),
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
                self.top_nav_search_bar.typeahead_results = Some(json.into_serde().unwrap());
            }
            Msg::InitiatedContributorsDownload => {
                self.has_initiated_contributors_download = true;
            }
            Msg::InitiatedTypeaheadRequest => {
                self.top_nav_search_bar.has_initiated_auto_complete_download = true;
            }
            Msg::TypeaheadOpen(v) => {
                self.top_nav_search_bar.is_typeahead_open = *v;
            }
            Msg::KeyDown(v) => match v {
                Some(key) => {
                    self.set_typeahead_active_index(key.clone());
                }
                None => {}
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

    pub fn top_nav_search_bar(&self) -> &TopNavSearchBar {
        &self.top_nav_search_bar
    }

    pub fn has_initiated_contributors_download(&self) -> &bool {
        &self.has_initiated_contributors_download
    }
}

impl State {
    fn increment_click(&mut self) {
        self.click_count.set(self.click_count.get() + 1);
    }

    fn set_path(&mut self, path: String) {
        self.path = path;
    }

    fn set_typeahead_active_index(&mut self, key: String) {
        match key.as_ref() {
            "ArrowDown" | "Down" => match self.top_nav_search_bar.typeahead_active_index {
                Some(index) => {
                    self.top_nav_search_bar.typeahead_active_index = Some(index + 1);
                }
                None => self.top_nav_search_bar.typeahead_active_index = Some(0),
            },
            "ArrowUp" | "Up" => match self.top_nav_search_bar.typeahead_active_index {
                Some(index) => {
                    self.top_nav_search_bar.typeahead_active_index = Some(index - 1);
                }
                None => self.top_nav_search_bar.typeahead_active_index = Some(0),
            },
            _ => {}
        }
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
