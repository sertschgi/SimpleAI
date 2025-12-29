use super::utils::*;

#[cfg(not(target_family = "wasm"))]
fn set_size() {
    use dioxus::desktop::{window, LogicalSize};
    window().set_max_inner_size(Some(LogicalSize::new(720, 512)));
    window().set_min_inner_size(Some(LogicalSize::new(720, 512)));
}

#[cfg(target_family = "wasm")]
fn set_size() {}

#[page(no_css = true)]
pub fn ProjectNav(id: uuid::Uuid) -> Element {
    set_size();
    rsx! {
        main {
            NavigationPage {
                NavigationLink { to: Route::Edit { id }, label: "edit", EditIcon {} }
                NavigationLink { to: Route::Editor { id }, label: "workflow editor", WorkflowIcon {} }
                NavigationLink { to: Route::Editor { id }, label: "onnx editor", ProjectsIcon {} }
            }
        }
    }
}
