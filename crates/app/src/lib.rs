#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate log;
extern crate models;

pub use crate::state::*;
pub use crate::store::*;
use crate::views::*;
use css_rs_macro::css;
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
    pub fn new(path: String) -> App {
        let state = State::new();
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
        let top_nav = self.render_top_nav();
        let side_nav = self.render_side_nav();
        let main = self.router.view(self.store.borrow().path()).unwrap();
        html! {
            <div id="app" class=MAIN_CONTAINER_STYLE>
                <div id="header">
                    { top_nav }
                </div>
                <div id="main">
                    <div id="side-nav">
                        { side_nav }
                    </div>
                    <div id="main-content">
                        { main }
                    </div>
                </div>
            </div>
        }
    }
}

/// Render the top nav
impl App {
    pub fn render_top_nav(&self) -> VirtualNode {
        match &self.store.borrow().top_nav.is_visible {
            true => {
                let store = Rc::clone(&self.store);
                TopNavBarView::new(store).render()
            }
            false => {
                html! { <div style="display:none;"></div> }
            }
        }
    }
}

/// Render the side nav
impl App {
    pub fn render_side_nav(&self) -> VirtualNode {
        match &self.store.borrow().side_nav.is_visible {
            true => {
                let store = Rc::clone(&self.store);
                SideNavBarView::new(store).render()
            }
            false => {
                html! { <div style="display:none;"></div> }
            }
        }
    }
}

static MAIN_CONTAINER_STYLE: &'static str = css! {"
:host {
  width: 100%;
  height: 100%;
}

:host > #header {
    position: relative;
    z-index: 2;
}

:host > #main {
    position: relative;
    z-index: 1;
}

:host > #main > #side-nav {
    float: left;
    width: 300px;
}

"};

#[route(
    path = "/",
    on_visit = hide_sidebar
)]
fn home_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    HomeView::new(Rc::clone(&store)).render()
}

#[route(
    path = "/companies/:short_cik",
    on_visit = on_company_visit_handler
)]
fn company_route(short_cik: String, store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    CompanyView::new(short_cik, Rc::clone(&store)).render()
}

fn on_company_visit_handler(short_cik: String, store: Provided<Rc<RefCell<Store>>>) {
    println!("hello");
    //    let store_1 = Rc::clone(&store);
    //    toggle_sidebar(true, store_1);
    //    let store_2 = Rc::clone(&store);
    //    get_company_filings(&short_cik, store_2);
}

fn hide_sidebar(store: Provided<Rc<RefCell<Store>>>) {
    println!("ww");
    toggle_sidebar(false, Rc::clone(&store));
}
fn toggle_sidebar(is_visible: bool, store: Rc<RefCell<Store>>) {
    // In order to check if the download has already been initiated, we must
    // wrap the possibility of a download attempt in a closure and pass it to
    // request_animation_frame. This is due to store already being mutably
    // borrowed, since this method will be called from the `Store.msg` function.
    //
    // TODO: Do this in `Store.msg` instead of needing to do it in every on_visit callback
    let raf_closure = Closure::wrap(Box::new(move || {
        store
            .borrow_mut()
            .msg(&Msg::SetSideNavVisibility(is_visible));
    }) as Box<FnMut()>);

    web_sys::window()
        .unwrap()
        .request_animation_frame(raf_closure.as_ref().unchecked_ref())
        .unwrap();

    // TODO: We don't want to repeatedly forget this closure and should instead figure out a place
    // to store it.
    // Maybe make our `Store`'s msg handler for Msg::SetPath call `on_visit` inside of a RAF..
    raf_closure.forget();
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

pub fn get_company_filings(short_cik: &String, store: Rc<RefCell<Store>>) {
    let callback = Closure::wrap(Box::new(move |json: JsValue| {
        store.borrow_mut().msg(&Msg::SetCompanyPageFilings(json));
    }) as Box<FnMut(JsValue)>);
    download_json(
        &*format!("http://localhost:8000/api/companies/{}/filings", short_cik),
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
