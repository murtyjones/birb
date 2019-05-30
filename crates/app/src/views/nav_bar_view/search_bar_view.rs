use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use crate::Msg;
use css_rs_macro::css;
use models::Company;
use wasm_bindgen::JsCast;

use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;

use crate::state::TypeaheadResponse;
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
        debug!("Key downed: {:?}", self.store.borrow().key_down());
        let store_for_onfocus = Rc::clone(&self.store);
        let store_for_onblur = Rc::clone(&self.store);
        let store_for_oninput = Rc::clone(&self.store);
        let store_for_typeahead_results = Rc::clone(&self.store);

        let typeahead_results = build_typeahead_results(store_for_typeahead_results);

        html! {
            <div class=TYPEAHEAD_CSS>
                <input
                    id="company-autocomplete"
                    type="text"
                    name="company"
                    autocomplete="off"
                    onfocus=move |event: web_sys::Event| {
                        store_for_onfocus.borrow_mut().msg(&Msg::TypeaheadOpen(true))
                    }
                    onblur=move |event: web_sys::Event| {
                        store_for_onblur.borrow_mut().msg(&Msg::TypeaheadOpen(false))
                    }
                    oninput=move |event: web_sys::Event| {
                        let value: String = event.target()
                            .expect("Couldn't unwrap event target")
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .expect("Couldn't convert into html element")
                            .value();
                        if value.len() > 0 {
                            store_for_oninput.borrow_mut().msg(&Msg::TypeaheadOpen(true));
                            store_for_oninput.borrow_mut().get_typeahead_results(value, Rc::clone(&store_for_oninput));
                        } else {
                            store_for_oninput.borrow_mut().msg(&Msg::TypeaheadOpen(false));
                        }
                    }
                />
                { typeahead_results }
            </div>
        }
    }
}

static TYPEAHEAD_CSS: &'static str = css! {"
:host {
  width: 250px;
  height: 20px;
  overflow-y: visible;
  color: black;
  margin: 0 auto;
}

:host > input {
  width: 100%;
  border: none;
  border-radius: 3px;
  padding: 5px;
  box-sizing: border-box;
}

:host > .typeahead-results > a {
  width: 100%;
  display: block;
  font-size: 12px;
  color: black;
  text-decoration: none;
  font-weight: normal;
  background: white;
  padding: 5px 10px;
  box-sizing: border-box;
  border-width: 1px 1px 0 1px;
  border-color: grey;
  border-style: solid;
}

:host > .typeahead-results > a:last-of-type {
  border-bottom-width: 1px;
}

:host > .typeahead-results > a:hover,
:host > .typeahead-results > a:focus,
:host > .typeahead-results > a.active {
  background: #DDD;
}

"};

fn build_typeahead_results(store: Rc<RefCell<Store>>) -> VirtualNode {
    match (
        store.borrow().is_typeahead_open(),
        store.borrow().typeahead_results(),
    ) {
        (true, Some(results)) => {
            let result_list = results
                .data
                .iter()
                .enumerate()
                .map(|(i, each)| {
                    let name: &str = each.company_name.as_str();
                    let link: String = format!("/companies/{}", each.short_cik.as_str());
                    let class: &str = match store.borrow().typeahead_active_index() {
                        Some(index) => match *index == i as i32 {
                            true => "active",
                            false => "inactive",
                        },
                        None => "inactive",
                    };
                    html! {
                        <a class={ class } href={ link }>{ name }</a>
                    }
                })
                .collect::<Vec<VirtualNode>>();
            html! {
                <div class="typeahead-results">{ result_list }<div>
            }
        }
        (true, None) => {
            html! { <div class="typeahead-results"><a style="cursor: default;">No Results</a><div> }
        }
        (false, ..) => {
            html! { <div style="display: none;"></div> }
        }
    }
}
