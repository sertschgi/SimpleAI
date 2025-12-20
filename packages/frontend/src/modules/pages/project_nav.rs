use super::utils::*;

#[page(no_css = true)]
pub fn ProjectNav(project_id: uuid::Uuid) -> Element {
    rsx! {
        main {
            NavigationPage {
                NavigationLink { to: Route::Editor {}, label: "onnx editor", ProjectsIcon {} }
            }
        }
    }
}
