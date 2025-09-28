// %%% components / nav_button.rs %%%

// %% includes %%
// use dioxus::{dioxus_core::AttributeValue, router::NavigationTarget};
use super::focus_button::*;
use super::search::Search;
use super::utils::*;
use std::collections::HashMap;

// %% main %%

#[component]
// #[component(no_css = true)]
pub fn FocusButtonArray(
    children: Element,
    #[props(default)] onfocus: Callback<()>,
    #[props(default)] onunfocus: Callback<()>,
    #[props(default)] onclick: Callback<MouseEvent>,
    #[props(default = use_signal(|| false))] focused: Signal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut section_contents_map = use_signal(HashMap::new);
    rsx! {
        nav {
            FocusButton {
                onfocus: move || {
                    section_contents_map.write().insert("search", rsx! {
                        Search {}
                        "Hello"
                    });
                },
                onunfocus: move || {
                    section_contents_map.write().remove("search");
                },
                SearchIcon {}
            }
        }
        section {
            for (_ , e) in section_contents_map() {
                {e}
            }
        }
    }
}
