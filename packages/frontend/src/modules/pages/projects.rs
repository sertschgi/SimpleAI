// %%% pages / start.rs %%%

// %% includes %%
use super::utils::*;
use dioxus::router::NavigationTarget;

// %% main %%
#[page]
pub fn Projects() -> Element {
    rsx! {
        main {
            h1 { class: "projects-heading", "Projects" }
            div { class: "projects-view" }
        }
    }
}
