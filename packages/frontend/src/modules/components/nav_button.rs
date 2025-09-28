// %%% components / nav_button.rs %%%

// %% includes %%
use super::utils::*;
use dioxus::router::NavigationTarget;

// %% main %%

#[item]
pub fn NavButton(
    children: Element,
    #[props(into)] to: NavigationTarget,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Link { to, class: "asdf",
            div { ..attributes,{children} }
        }
    }
}
