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
        let nav_bar = NavBarView::new(ActivePage::Home, Rc::clone(&self.store)).render();

        html! {
            <div>
                { nav_bar }
            </div>
        }
    }
}
