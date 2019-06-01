use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::Cell;
use std::rc::Rc;

mod top_nav;
use top_nav::TopNavSearchBar;

mod msg;
pub use self::msg::Msg;
use crate::state::top_nav::TypeaheadResponse;
use core::borrow::BorrowMut;

#[derive(Serialize, Deserialize)]
pub struct State {
    path: String,
    top_nav_search_bar: TopNavSearchBar,
}

impl State {
    pub fn new() -> State {
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
                    self.handle_typeahead_enter_key(key.clone());
                    self.handle_typeahead_escape_key(key.clone());
                    self.handle_typeahead_arrow_keys(key.clone());
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

    /// If the enter key is pressed and the typeahead is open,
    /// go to the company page of the active menu item
    fn handle_typeahead_enter_key(&mut self, key: String) {
        if key == "Enter" {
            let typeahead_active_index = self.top_nav_search_bar.typeahead_active_index;
            let is_typeahead_open = self.top_nav_search_bar.is_typeahead_open;
            let typeahead_results = &self.top_nav_search_bar.typeahead_results;
            match (is_typeahead_open, typeahead_results, typeahead_active_index) {
                (true, Some(response), Some(index)) => {
                    if response.data.len() > 0 {
                        let company = &response.data[index as usize];
                        let link = format!("/companies/{}", company.short_cik);
                        self.borrow_mut().msg(&Msg::SetPath(link));
                    }
                }
                _ => {}
            }
        }
    }

    /// If the escape key is pressed and the typeahead is open,
    /// close it
    fn handle_typeahead_escape_key(&mut self, key: String) {
        let TopNavSearchBar {
            is_typeahead_open, ..
        } = self.top_nav_search_bar;
        if is_typeahead_open && key == "Escape" || is_typeahead_open && key == "Esc"
        /* IE/Edge */
        {
            self.borrow_mut().msg(&Msg::TypeaheadOpen(false))
        }
    }

    /// If search results exist and an arrow key is pressed,
    /// increment or decrement which menu item is focused
    fn handle_typeahead_arrow_keys(&mut self, key: String) {
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
