use crate::store::Store;
use css_rs_macro::css;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

pub struct CompanyView {
    store: Rc<RefCell<Store>>,
}

impl CompanyView {
    pub fn new(store: Rc<RefCell<Store>>) -> CompanyView {
        CompanyView { store }
    }
}

impl View for CompanyView {
    fn render(&self) -> VirtualNode {
        html! {
            <div class=COMPANY_CSS>
                This is a lovely company page
            </div>
        }
    }
}

static COMPANY_CSS: &'static str = css! {"
:host {
}
"};
