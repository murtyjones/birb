#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate log;
extern crate models;

pub use crate::state::*;
pub use crate::store::*;
use crate::views::*;
use router_rs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
pub use virtual_dom_rs::VirtualNode;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

mod state;
mod store;
mod views;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "downloadJson")]
    pub fn download_json(path: &str, callback: &js_sys::Function);
}

pub struct App {
    pub store: Rc<RefCell<Store>>,
    router: Rc<Router>,
}

impl App {
    pub fn new(count: u32, path: String) -> App {
        let state = State::new(count);
        let store = Rc::new(RefCell::new(Store::new(state)));

        store.borrow_mut().msg(&Msg::SetPath(path));

        let router = make_router(Rc::clone(&store));

        store.borrow_mut().set_router(Rc::clone(&router));

        App { store, router }
    }

    pub fn from_state_json(json: &str) -> App {
        let state = State::from_json(json);
        let store = Rc::new(RefCell::new(Store::new(state)));

        let router = make_router(Rc::clone(&store));

        store.borrow_mut().set_router(Rc::clone(&router));

        let path = store.borrow().path().to_string();

        store.borrow_mut().msg(&Msg::SetPath(path));

        App { store, router }
    }
}

impl App {
    pub fn render(&self) -> VirtualNode {
        self.router.view(self.store.borrow().path()).unwrap()
    }
}

#[route(path = "/")]
fn home_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    HomeView::new(Rc::clone(&store)).render()
}

#[route(path = "/company/:short_cik")]
fn company_route(short_cik: String, store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    CompanyView::new(Rc::clone(&store)).render()
}

pub fn download_typeahead_json(substr: String, store: Rc<RefCell<Store>>) {
    let callback = Closure::wrap(Box::new(move |json: JsValue| {
        store.borrow_mut().msg(&Msg::SetTypeaheadJson(json));
    }) as Box<FnMut(JsValue)>);
    download_json(
        &*format!("http://localhost:8000/api/autocomplete/{}", substr),
        callback.as_ref().unchecked_ref(),
    );
    callback.forget();
}

fn make_router(store: Rc<RefCell<Store>>) -> Rc<Router> {
    let mut router = Router::default();

    router.provide(store);

    router.set_route_handlers(create_routes![home_route, company_route]);

    Rc::new(router)
}

#[cfg(test)]
mod tests {
    use super::*;
}
