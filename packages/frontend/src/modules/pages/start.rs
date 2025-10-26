// %%% pages / start.rs %%%

// %% includes %%
use super::utils::*;

// %% main %%
#[page]
pub fn Start() -> Element {
    rsx! {
        main {
            NavigationPage {
                NavButton { class: "editor", to: Route::Projects {},
                    ProjectsIcon {}
                    p { "projects" }
                }
                NavButton { class: "new", to: Route::New {},
                    NewIcon {}
                    p { "new project" }
                }
            }
        }
    }
}
