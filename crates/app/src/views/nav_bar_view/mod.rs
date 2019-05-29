use crate::store::Store;
use css_rs_macro::css;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

mod nav_bar_item_view;
use self::nav_bar_item_view::NavBarItemView;
mod search_bar_view;
use search_bar_view::SearchBarView;

pub struct NavBarView {
    active_page: ActivePage,
    store: Rc<RefCell<Store>>,
}

impl NavBarView {
    pub fn new(active_page: ActivePage, store: Rc<RefCell<Store>>) -> NavBarView {
        NavBarView { active_page, store }
    }
}

pub enum ActivePage {
    Home,
    Contributors,
}

impl View for NavBarView {
    fn render(&self) -> VirtualNode {
        // Links
        let home = NavBarItemView::new("/", "Isomorphic Web App", "");
        let contributors =
            NavBarItemView::new("/contributors", "Contributors", "margin-left: auto;");

        // Search bar
        let search_bar = SearchBarView::new(Rc::clone(&self.store));

        html! {
            <div class=NAV_BAR_CSS>
                { home.render() }
                { search_bar.render() }
                { contributors.render() }
            </div>
        }
    }
}

static NAV_BAR_CSS: &'static str = css! {"
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
