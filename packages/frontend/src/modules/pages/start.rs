use super::utils::*;
use dioxus::desktop::{window, LogicalSize};

#[page]
pub fn Start() -> Element {
    window().set_max_inner_size(Some(LogicalSize::new(720, 512)));
    window().set_min_inner_size(Some(LogicalSize::new(720, 512)));
    rsx! {
        main {
            NavigationPage {
                NavigationLink { to: Route::Projects {}, label: "projects", ProjectsIcon {} }
                NavigationLink { to: Route::New {}, label: "new project", NewIcon {} }
            }
        }
    }
}
