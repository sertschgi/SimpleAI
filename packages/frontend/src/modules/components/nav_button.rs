use super::utils::*;
use dioxus::{core::AttributeValue, router::NavigationTarget};

#[item]
pub fn NavButton(
    children: Element,
    #[props(into)] to: NavigationTarget,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut other_classes = String::new();
    if let Some(pos) = attributes.iter().position(|x| x.name == "class") {
        let value = attributes.remove(pos).value;
        if let AttributeValue::Text(text) = value {
            other_classes = text;
        }
    }
    rsx! {
        Link { class: "{other_classes}", to, attributes, {children} }
    }
}
