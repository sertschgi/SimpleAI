use super::utils::*;
use super::{breadcrumbs::Breadcrumbs, window_decorations::WindowDecorations};

#[item]
pub fn TopNav() -> Element {
    rsx! {
        div {
            Breadcrumbs {}
            WindowDecorations {}
        }
    }
}

#[item]
pub fn TopNavLayout() -> Element {
    rsx! {
        div {
            TopNav {}
            article { Outlet::<Route> {} }
        }
    }
}
