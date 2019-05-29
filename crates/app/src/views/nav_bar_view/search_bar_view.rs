use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use crate::Msg;
use wasm_bindgen::JsCast;

use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct SearchBarView {
    store: Rc<RefCell<Store>>,
}

impl SearchBarView {
    pub fn new(store: Rc<RefCell<Store>>) -> SearchBarView {
        SearchBarView { store }
    }
}

impl View for SearchBarView {
    fn render(&self) -> VirtualNode {
        let store = Rc::clone(&self.store);

        let autocomplete_dropdown = match self.store.borrow().autocomplete_results() {
            Some(results) => {
                html! {
                    <div>
                        Dropdown content!
                    </div>
                }
            }
            None => {
                html! { <div style="display: none;"></div> }
            }
        };

        html! {
            <div>
                <input
                    id="company-autocomplete"
                    type="text"
                    name="company"
                    autocomplete="off"
                    oninput=move |event: web_sys::Event| {
                        let value: String = event.target()
                            .unwrap()
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .unwrap()
                            .value();
                        store.borrow_mut().get_autocomplete(value, Rc::clone(&store));
                    }
                />
                { autocomplete_dropdown }
            </div>
        }
    }
}
