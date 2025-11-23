use super::utils::*;
use super::{breadcrumbs::Breadcrumbs, window_decorations::WindowDecorations};

#[cfg(not(target_family = "wasm"))]
use dioxus::desktop::window;

#[cfg(not(target_family = "wasm"))]
#[item]
pub fn TopNav() -> Element {
    rsx! {
        div { onmousedown: move |_| { window().drag() },
            Breadcrumbs {}
            WindowDecorations {}
        }
    }
}

#[cfg(target_family = "wasm")]
#[item]
pub fn TopNav() -> Element {
    rsx! {
        div {
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
