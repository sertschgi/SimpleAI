use super::utils::*;

#[page(no_css = true)]
pub fn ProjectNav(id: uuid::Uuid) -> Element {
    rsx! {
        main {
            NavigationPage {
                NavigationLink { to: Route::Editor { id }, label: "onnx editor", ProjectsIcon {} }
            }
        }
    }
}
