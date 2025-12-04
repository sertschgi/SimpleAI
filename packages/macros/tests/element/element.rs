use dioxus::prelude::*;
use simple_ai_macros::*;

#[element(kind = "entry", no_css = true)]
pub fn TestElementEntry() -> Element {
    rsx! {
        main {
        }
    }
}

#[element(kind = "item", no_css = true)]
pub fn TestElementItem() -> Element {
    rsx! {
        main {
        }
    }
}

#[element(kind = "page", no_css = true)]
pub fn TestElementPage() -> Element {
    rsx! {
        main {
        }
    }
}
