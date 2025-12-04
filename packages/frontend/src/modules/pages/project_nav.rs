use super::utils::*;

#[page(no_css = true)]
pub fn ProjectNav() -> Element {
    rsx! {
        main {
            NavigationPage {
                NavigationLink { to: Route::Editor {}, label: "onnx editor", ProjectsIcon {} }
            }
        }
    }
}
