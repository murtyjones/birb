use crate::download_autocomplete_json;
use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use crate::Msg;
use wasm_bindgen::JsCast;

use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct HomeView {
    store: Rc<RefCell<Store>>,
}

impl HomeView {
    pub fn new(store: Rc<RefCell<Store>>) -> HomeView {
        HomeView { store }
    }
}

impl View for HomeView {
    fn render(&self) -> VirtualNode {
        let nav_bar = NavBarView::new(ActivePage::Home).render();

        let store = Rc::clone(&self.store);
        // To avoid moving store into the closure:
        // More details: https://github.com/rustwasm/wasm-bindgen/issues/1269
        let store_ref = Some(store);

        let click_count = self.store.borrow().click_count();
        let click_count = &*click_count.to_string();
        let click_component = html! { <strong style="font-size: 30px">{ click_count }</strong> };

        let autocomplete_dropdown = match self.store.borrow().autocomplete_results() {
            Some(results) => {
                html! { <div>Dropdown content!</div> }
            }
            None => {
                html! { <div style="display: none;"></div> }
            }
        };

        let mut autocomplete_timeout: Option<i32> = None;

        html! {
            <div>
                  { nav_bar }

                  <span> The button has been clicked: { click_component } times! </span>
                  <input
                      id="company-autocomplete"
                      type="text"
                      name="company"
                      autocomplete="off"
                      oninput=move |event: web_sys::Event| {
                        match autocomplete_timeout {
                            Some(timeout) => {
                                web_sys::window().unwrap().clear_timeout_with_handle(timeout);
                            }
                            None => {}
                        }
                        let debounced_request = Closure::wrap(Box::new(move |_: web_sys::Event| {
                            let value: String = event.target()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value();
                            // TODO figure out how to actually call this
                            // Current error: expected an `FnMut<(web_sys::Event,)>` closure, found `[closure@crates/app/src/views/home_view.rs:58:31: 79:24 autocomplete_timeout:std::option::Option<i32>, store_ref:std::option::Option<std::rc::Rc<std::cell::RefCell<store::Store>>>]`
                            // download_autocomplete_json(value, store_ref.take().unwrap());
                        }) as Box<FnMut(_)>);
                        autocomplete_timeout = Some(web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                            debounced_request.as_ref().unchecked_ref(),
                            200 // only make 200ms after using stops typing
                        ).unwrap());
                        debounced_request.forget();
                      }
                  />
                  { autocomplete_dropdown }
            </div>
        }
    }
}
