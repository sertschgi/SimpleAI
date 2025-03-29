use crate::prelude::*;
use dioxus::router::prelude::*;

#[component]
pub fn NavButton(
    children: Element,
    class: Option<String>,
    #[props(into)] to: NavigationTarget,
) -> Element {
    let class_unw = class.unwrap_or_default();
    rsx! {
        Link {
            class: "NavbarButton {class_unw}",
            to: to,
            div {
                {children}
            }
        }
    }
}
