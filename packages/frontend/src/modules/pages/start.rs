// %%% pages / start.rs %%%

// %% includes %%
use super::utils::*;
use dioxus::router::prelude::*;

// %% main %%
#[page]
pub fn Start(
    #[props(into)] search_route: NavigationTarget,
    #[props(into)] new_route: NavigationTarget,
    #[props(into)] editor_route: NavigationTarget,
) -> Element {
    rsx! {
        main {
            article {
                section {
                    NavButton { class: "search", to: search_route, SearchIcon {}}
                    NavButton { class: "new", to: new_route, NewIcon {} }
                    NavButton { class: "editor", to: editor_route, EditorIcon {} }
                }
            }
        }
    }
}
