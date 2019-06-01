use crate::download_typeahead_json;
use crate::state::Msg;
use crate::state::State;

use core::borrow::BorrowMut;
use router_rs::prelude::Router;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct Store {
    state: StateWrapper,
    after_route: Option<Box<Fn(&str) -> ()>>,
    router: Option<Rc<Router>>,
    listeners: Vec<Box<Fn() -> ()>>,
}

impl Store {
    pub fn new(state: State) -> Store {
        Store {
            state: StateWrapper(state),
            after_route: None,
            router: None,
            listeners: vec![],
        }
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            // TODO: Right now `on_visit` cannot borrow store since it's already borrowed.
            // So we might want to explore wraping our `on_visit` in requestAnimationFrame
            // so that by the time it runs we are no longer borrowing store ... or something ...
            Msg::SetPath(path) => {
                if let Some(router) = &self.router {
                    if let Some(route_handler) = router.matching_routerhandler(path.as_str()) {
                        route_handler.on_visit(path.as_str());
                    }
                }

                self.state.msg(msg);
                if let Some(after_route) = &self.after_route {
                    debug!("hmmmm");
                    after_route(path.as_str());
                }
            }
            Msg::KeyDown(v) => {
                match v {
                    Some(key) => {
                        self.handle_typeahead_enter_key(key.clone());
                    }
                    None => {}
                }
                self.state.msg(msg);
            }
            _ => self.state.msg(msg),
        }

        // Whenever we update state we'll let all of our listeners know that state was updated
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.listeners.push(callback)
    }

    pub fn set_after_route(&mut self, after_route: Box<Fn(&str) -> ()>) {
        self.after_route = Some(after_route);
    }

    pub fn set_router(&mut self, router: Rc<Router>) {
        self.router = Some(router);
    }
}

impl Store {
    pub fn get_typeahead_results(&self, value: String, store: Rc<RefCell<Store>>) {
        download_typeahead_json(value, store);
    }
}

/// typeahead stuff
impl Store {
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
}

impl Deref for Store {
    type Target = State;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.state
    }
}

struct StateWrapper(State);

impl Deref for StateWrapper {
    type Target = State;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl StateWrapper {
    fn msg(&mut self, msg: &Msg) {
        self.0.msg(msg)
    }
}
