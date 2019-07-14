use crate::store::Store;
use css_rs_macro::css;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

pub struct CompanyView {
    store: Rc<RefCell<Store>>,
    short_cik: String,
}

impl CompanyView {
    pub fn new(short_cik: String, store: Rc<RefCell<Store>>) -> CompanyView {
        CompanyView { store, short_cik }
    }
}

impl View for CompanyView {
    fn render(&self) -> VirtualNode {
        let id = &*self.short_cik;
        html! {
            <div class=COMPANY_CSS>
                This is a lovely company page for { id }!
            </div>
        }
    }
}

static COMPANY_CSS: &'static str = css! {"
:host {
}
"};
