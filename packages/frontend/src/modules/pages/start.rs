// %%% pages / start.rs %%%

// %% includes %%
use super::utils::*;

// %% main %%
#[page]
pub fn Start() -> Element {
    rsx! {
        main {
            NavigationPage {
                NavigationLink { to: Route::Projects {}, label: "projects", ProjectsIcon {} }
                NavigationLink { to: Route::New {}, label: "new project", NewIcon {} }
            }
        }
    }
}
