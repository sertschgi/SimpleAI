use dioxus::prelude::*;
use simple_ai_macros::*;

#[item(no_css = true)]
pub fn TestItemDxElement() -> Element {
    rsx! {
        Link { to: Route::Index {} }
    }
}
