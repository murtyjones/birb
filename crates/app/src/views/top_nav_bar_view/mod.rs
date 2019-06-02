use crate::store::Store;
use css_rs_macro::css;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

mod top_nav_bar_item_view;
use self::top_nav_bar_item_view::TopNavBarItemView;
mod search_bar_view;
use search_bar_view::SearchBarView;

pub struct TopNavBarView {
    store: Rc<RefCell<Store>>,
}

impl TopNavBarView {
    pub fn new(store: Rc<RefCell<Store>>) -> TopNavBarView {
        TopNavBarView { store }
    }
}

impl View for TopNavBarView {
    fn render(&self) -> VirtualNode {
        // Links
        let home = TopNavBarItemView::new("/", "birb", "");

        // Search bar
        let search_bar = SearchBarView::new(Rc::clone(&self.store));

        html! {
            <div class=NAV_BAR_CSS>
                { home.render() }
                { search_bar.render() }
            </div>
        }
    }
}

static NAV_BAR_CSS: &'static str = css! {"
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
