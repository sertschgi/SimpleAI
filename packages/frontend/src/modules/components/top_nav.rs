// %%% components / top_nav.rs %%%
// %% includes %%
use super::utils::*;
use super::{breadcrumbs::Breadcrumbs, window_decorations::WindowDecorations};

// %% main %%
#[item]
pub fn TopNav() -> Element {
    rsx! {
        div {
            Breadcrumbs {}
            WindowDecorations {}
        }
    }
}

pub fn TopNavLayout() -> Element {
    rsx! {
        TopNav {}
        Outlet::<Route> {}
    }
}
