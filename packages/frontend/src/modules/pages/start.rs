// %%% pages / start.rs %%%

// %% includes %%
use super::utils::*;
use dioxus::router::NavigationTarget;

// %% main %%
#[page]
pub fn Start(
    #[props(into)] search_route: NavigationTarget,
    #[props(into)] new_route: NavigationTarget,
    #[props(into)] editor_route: NavigationTarget,
) -> Element {
    rsx! {
        main {
            div { class: "button-container",
                NavButton { class: "search", to: search_route,
                    SearchIcon {}
                    p { "search" }
                }
                NavButton { class: "new", to: new_route,
                    NewIcon {}
                    p { "new" }
                }
                NavButton { class: "editor", to: editor_route,
                    EditorIcon {}
                    p { "editor" }
                }
            }
        }
    }
}
