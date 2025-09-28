use dioxus::dioxus_core::{Attribute, AttributeValue};
pub fn add_to_classes(class: &str, attributes: &mut Vec<Attribute>) {
    for a in attributes {
        if a.name == "class" {
            a.value = AttributeValue::Text(format!("{} {:?}", class, a.value));
        }
    }
}
