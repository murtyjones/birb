use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use crate::Msg;
use css_rs_macro::css;
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

        let will_this_work: Vec<VirtualNode> = vec![
            html! { <a>1</a> },
            html! { <a>2</a> },
            html! { <a>3</a> },
            html! { <a>4</a> },
            html! { <a>5</a> },
            html! { <a>6</a> },
            html! { <a>7</a> },
            html! { <a>8</a> },
        ];

        let typeahead_results = match self.store.borrow().autocomplete_results() {
            Some(results) => {
                html! {
                    <div class="typeahead-results">
                        { will_this_work }
                    </div>
                }
            }
            None => {
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
                    oninput=move |event: web_sys::Event| {
                        let value: String = event.target()
                            .unwrap()
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .unwrap()
                            .value();
                        store.borrow_mut().get_autocomplete(value, Rc::clone(&store));
                    }
                />
                { typeahead_results }
            </div>
        }
    }
}

static TYPEAHEAD_CSS: &'static str = css! {"
:host {
  height: 20px;
  width: 100px;
  overflow-y: visible;
  color: black;
}

:host > input {
  border: none;
  border-radius: 3px;
  padding: 5px;
}

:host > .typeahead-results > a {
  display: block;
  background: white;
  padding: 5px 10px;
  border-width: 1px 1px 0 1px;
  border-color: grey;
  border-style: solid;
}

:host > .typeahead-results > a:last-child {
  border-bottom-width: 1px;
}

"};
