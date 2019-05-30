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
    path: String,
    top_nav_search_bar: TopNavSearchBar,
}

impl State {
    pub fn new(count: u32) -> State {
        State {
            path: "/".to_string(),
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
            Msg::SetPath(path) => self.set_path(path.to_string()),
            Msg::SetTypeaheadJson(json) => {
                self.top_nav_search_bar.typeahead_results = Some(json.into_serde().unwrap());
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

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn top_nav_search_bar(&self) -> &TopNavSearchBar {
        &self.top_nav_search_bar
    }
}

impl State {
    fn set_path(&mut self, path: String) {
        self.path = path;
    }

    fn set_typeahead_active_index(&mut self, key: String) {
        match &self.top_nav_search_bar.typeahead_results {
            Some(response) => match key.as_ref() {
                "ArrowDown" | "Down" => match self.top_nav_search_bar.typeahead_active_index {
                    Some(index) => {
                        // go down one item in the list (or to the top of the list if at i == last index
                        let last_index = (response.data.len() - 1) as i32;
                        let new_index = if index + 1 > last_index { 0 } else { index + 1 };
                        self.top_nav_search_bar.typeahead_active_index = Some(new_index);
                    }
                    None => self.top_nav_search_bar.typeahead_active_index = Some(0),
                },
                "ArrowUp" | "Up" => match self.top_nav_search_bar.typeahead_active_index {
                    Some(index) => {
                        // go up one item in the list (or to the bottom of the list if at i == 0
                        let last_index = (response.data.len() - 1) as i32;
                        let new_index = if index - 1 < 0 { last_index } else { index - 1 };
                        self.top_nav_search_bar.typeahead_active_index = Some(new_index);
                    }
                    None => self.top_nav_search_bar.typeahead_active_index = Some(0),
                },
                _ => {}
            },
            None => {
                self.top_nav_search_bar.typeahead_active_index = None;
            }
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
        let state_json =
            r#"{"click_count":5,"path":"/","has_initiated_auto_complete_download":false,}"#;

        let state = State::from_json(state_json);
        assert_eq!(&state.to_json(), state_json);
    }
}
