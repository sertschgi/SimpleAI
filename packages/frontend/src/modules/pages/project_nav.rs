use super::utils::*;

#[page(no_css = true)]
pub fn ProjectNav() -> Element {
    rsx! {
        main {
            NavigationPage {
                NavButton { to: Route::Projects {},
                    ProjectsIcon {}
                    p { "onnx-editor" }
                }
                NavButton { to: Route::Projects {},
                    ProjectsIcon {}
                    p { "export" }
                }
            }
        }
    }
}
