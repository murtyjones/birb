use css_rs_macro::css;
use virtual_dom_rs::prelude::*;

pub struct TopNavBarItemView {
    path: &'static str,
    text: &'static str,
    style: &'static str,
}

impl TopNavBarItemView {
    pub fn new(path: &'static str, text: &'static str, style: &'static str) -> TopNavBarItemView {
        TopNavBarItemView { path, text, style }
    }
}

impl View for TopNavBarItemView {
    fn render(&self) -> VirtualNode {
        html! {
            <a
             href=self.path
             style=self.style
             class=NAV_BAR_ITEM_CSS
            >
              { self.text }
            </a>
        }
    }
}

static NAV_BAR_ITEM_CSS: &'static str = css! {"
:host {
    border-bottom: solid transparent 3px;
    cursor: pointer;
    color: #E7CEF2;
    text-decoration: none;
}

:host:hover {
    border-bottom: solid #E7CEF2 3px;
}
"};
