use super::utils::*;

#[item]
pub fn LabeledBox(name: String, kind: String, required: bool, placeholder: String) -> Element {
    rsx! {
        div {
            h5 { {name.clone()} }
            input {
                name,
                required,
                placeholder,
                r#type: kind,
            }
        }
    }
}
