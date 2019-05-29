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

        let click_count = self.store.borrow().click_count();
        let click_count = &*click_count.to_string();
        let click_component = html! { <strong style="font-size: 30px">{ click_count }</strong> };

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
                { nav_bar }
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
