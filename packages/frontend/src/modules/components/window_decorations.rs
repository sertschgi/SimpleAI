use super::utils::*;

#[cfg(not(target_family = "wasm"))]
use dioxus::desktop::window;

use dioxus::core::AttributeValue;

#[cfg(not(target_family = "wasm"))]
#[item]
pub fn WindowDecorations() -> Element {
    rsx! {
        div {
            section {
                article { class: "wrapper", onclick: |_| { window().close() }, CloseIcon {} }
            }
        }
    }
}

#[cfg(target_family = "wasm")]
#[item]
pub fn WindowDecorations() -> Element {
    rsx! {
        div {
            section {
            }
        }
    }
}
