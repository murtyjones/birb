use crate::store::Store;
use css_rs_macro::css;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

pub struct SideNavBarView {
    store: Rc<RefCell<Store>>,
}

impl SideNavBarView {
    pub fn new(store: Rc<RefCell<Store>>) -> SideNavBarView {
        SideNavBarView { store }
    }
}

impl View for SideNavBarView {
    fn render(&self) -> VirtualNode {
        html! {
            <div class=SIDE_NAV_BAR_CSS>
                wow!
            </div>
        }
    }
}

static SIDE_NAV_BAR_CSS: &'static str = css! {"
:host {
    align-items: center;
    background: #592E56;
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
