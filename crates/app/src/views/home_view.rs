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

        let autocomplete_dropdown = if self.store.borrow().click_count() > 1000 {
            html! { <div>Dropdown content!</div> }
        } else {
            html! { <span></span> }
        };

        let mut autocomplete_timeout: Option<i32> = None;

        html! {
        <div>

          { nav_bar }

          <span> The button has been clicked: { click_component } times! </span>
          <input
              type="text"
              name="company"
              autocomplete="off"
              oninput=move |_: web_sys::Event| {
                match autocomplete_timeout {
                    Some(timeout) => {
                        web_sys::window().unwrap().clear_timeout_with_handle(timeout);
                        let debounced_request = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                            debug!("Debounced!");
                        }) as Box<FnMut(_)>);
                        autocomplete_timeout = Some(web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                            debounced_request.as_ref().unchecked_ref(),
                            3_000
                        ).unwrap());
                        debounced_request.forget();
                    }
                    None => {
                        let debounced_request = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                            debug!("Debounced!");
                        }) as Box<FnMut(_)>);
                        autocomplete_timeout = Some(web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                            debounced_request.as_ref().unchecked_ref(),
                            3_000
                        ).unwrap());
                        debounced_request.forget();
                    }
                }
              }
          />
          { autocomplete_dropdown }
        </div>
        }
    }
}
