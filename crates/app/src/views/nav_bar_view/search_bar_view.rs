use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use crate::Msg;
use css_rs_macro::css;
use models::Company;
use wasm_bindgen::JsCast;

use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;

use crate::state::AutoCompleteResponse;
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
        debug!(
            "Typehead open: {:?}",
            self.store.borrow().is_typeahead_open()
        );
        let store_for_onfocus = Rc::clone(&self.store);
        let store_for_onblur = Rc::clone(&self.store);
        let store_for_oninput = Rc::clone(&self.store);

        let will_this_work: Vec<VirtualNode> = vec![
            html! { <a href="/">1</a> },
            html! { <a href="/">2</a> },
            html! { <a href="/">3</a> },
            html! { <a href="/">4</a> },
            html! { <a href="/">5</a> },
            html! { <a href="/">6</a> },
            html! { <a href="/">7</a> },
            html! { <a href="/">8</a> },
        ];

        let typeahead_results: VirtualNode = match (
            self.store.borrow().is_typeahead_open(),
            self.store.borrow().autocomplete_results(),
        ) {
            (true, Some(results)) => {
                let result_list = results
                    .data
                    .iter()
                    .map(|x| {
                        let name: &str = x.company_name.as_str();
                        let link: String = format!("/companies/{}", x.short_cik.as_str());
                        html! {
                            <a href={ link }>{ name }</a>
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
        };

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
                            .unwrap()
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .unwrap()
                            .value();
                        store_for_oninput.borrow_mut().get_typeahead_results(value, Rc::clone(&store_for_oninput));
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
:host > .typeahead-results > a:focus {
  background: #DDD;
}

"};
