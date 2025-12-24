use super::utils::*;

#[page(no_css = true)]
pub fn ProjectNav(id: uuid::Uuid) -> Element {
    rsx! {
        main {
            NavigationPage {
                NavigationLink { to: Route::Edit { id }, label: "edit", EditIcon {} }
                NavigationLink { to: Route::Editor { id }, label: "onnx editor", ProjectsIcon {} }
            }
        }
    }
}
