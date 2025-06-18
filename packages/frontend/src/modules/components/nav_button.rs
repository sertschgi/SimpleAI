// %%% components / nav_button.rs %%%

// %% includes %%
use super::utils::*;
use dioxus::router::prelude::*;

// %% main %%

#[component]
pub fn NavButton(
    children: Element,
    class: Option<String>,
    #[props(into)] to: NavigationTarget,
) -> Element {
    let class_unw = class.unwrap_or_default();
    rsx! {
        Link {
            class: "NavbarButton button {class_unw}",
            to: to,
            div {
                {children}
            }
        }
    }
}
