use super::utils::*;
use dioxus::router::NavigationTarget;

#[item]
pub fn NavigationLink(
    #[props(into)] to: NavigationTarget,
    #[props(default)] label: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            button {
                Link { to, {children} }
            }
            label { "{label}" }
        }

    }
}
