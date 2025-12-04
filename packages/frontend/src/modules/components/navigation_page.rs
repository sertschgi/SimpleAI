use super::utils::*;

#[item]
pub fn NavigationPage(children: Element) -> Element {
    rsx! {
        main {
            div { class: "button-container", {children} }
        }
    }
}
