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
                Hi!
            </div>
        }
    }
}

static COMPANY_CSS: &'static str = css! {"
:host {
    align-items: center;
    background: linear-gradient(267deg,#2a38ef,#200994 50%,#1c2dab);
    color: white;
    display: flex;
    font-family: Helvetica;
    font-size: 20px;
    font-weight: bold;
    height: 50px;
    padding-left: 30px;
    padding-right: 30px;
}
"};
