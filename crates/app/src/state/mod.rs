mod side_nav;
mod top_nav;

use serde::{Deserialize, Serialize};
use serde_json;
use side_nav::SideNav;
use std::cell::Cell;
use std::rc::Rc;
use top_nav::TopNav;
use top_nav::TopNavSearchBar;

mod msg;
pub use self::msg::Msg;
use core::borrow::BorrowMut;

#[derive(Serialize, Deserialize)]
pub struct State {
    path: String,
    pub top_nav: TopNav,
    pub side_nav: SideNav,
}

impl State {
    pub fn new() -> State {
        State {
            path: "/".to_string(),
            top_nav: TopNav::new(),
            side_nav: SideNav::new(),
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
                self.top_nav.search_bar.typeahead_results = Some(json.into_serde().unwrap());
            }
            Msg::InitiatedTypeaheadRequest => {
                self.top_nav.search_bar.has_initiated_auto_complete_download = true;
            }
            Msg::TypeaheadOpen(v) => {
                self.top_nav.search_bar.is_typeahead_open = *v;
            }
            Msg::KeyDown(v) => match v {
                Some(key) => {
                    self.handle_typeahead_escape_key(key.clone());
                    self.handle_typeahead_arrow_keys(key.clone());
                }
                None => {}
            },
            Msg::Click(target) => match target {
                Some(element) => {
                    self.handle_typeahead_blur_click(element.clone());
                }
                None => {}
            },
            Msg::SetSideNavVisibility(v) => self.set_side_nav_visibility(*v),
        };
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn top_nav(&self) -> &TopNav {
        &self.top_nav
    }
}

impl State {
    fn set_path(&mut self, path: String) {
        self.path = path;
    }

    /// If the escape key is pressed and the typeahead is open,
    /// close it
    fn handle_typeahead_escape_key(&mut self, key: String) {
        let TopNavSearchBar {
            is_typeahead_open, ..
        } = self.top_nav.search_bar;
        if is_typeahead_open && key == "Escape" || is_typeahead_open && key == "Esc"
        /* IE/Edge */
        {
            self.borrow_mut().msg(&Msg::TypeaheadOpen(false))
        }
    }

    /// If search results exist and an arrow key is pressed,
    /// increment or decrement which menu item is focused
    fn handle_typeahead_arrow_keys(&mut self, key: String) {
        match &self.top_nav.search_bar.typeahead_results {
            Some(response) => match key.as_ref() {
                "ArrowDown" | "Down" => match self.top_nav.search_bar.typeahead_active_index {
                    Some(index) => {
                        // go down one item in the list (or to the top of the list if at i == last index
                        let last_index = (response.data.len() - 1) as i32;
                        let new_index = if index + 1 > last_index { 0 } else { index + 1 };
                        self.top_nav.search_bar.typeahead_active_index = Some(new_index);
                    }
                    None => self.top_nav.search_bar.typeahead_active_index = Some(0),
                },
                "ArrowUp" | "Up" => match self.top_nav.search_bar.typeahead_active_index {
                    Some(index) => {
                        // go up one item in the list (or to the bottom of the list if at i == 0
                        let last_index = (response.data.len() - 1) as i32;
                        let new_index = if index - 1 < 0 { last_index } else { index - 1 };
                        self.top_nav.search_bar.typeahead_active_index = Some(new_index);
                    }
                    None => self.top_nav.search_bar.typeahead_active_index = Some(0),
                },
                _ => {}
            },
            None => {
                self.top_nav.search_bar.typeahead_active_index = None;
            }
        }
    }
}

impl State {
    fn handle_typeahead_blur_click(&mut self, element: web_sys::Element) {
        if !element.class_name().contains("company-autocomplete")
            || element.class_name().contains("company-autocomplete-result")
        {
            self.borrow_mut().msg(&Msg::TypeaheadOpen(false))
        }
    }
}

impl State {
    fn set_side_nav_visibility(&mut self, is_visible: bool) {
        self.side_nav.is_visible = is_visible;
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
