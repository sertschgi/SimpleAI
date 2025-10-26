use super::utils::*;

#[page]
pub fn New() -> Element {
    rsx! {
        main {
            form {
                LabeledBox {
                    name: "name",
                    kind: "text",
                    required: true,
                    placeholder: "SampleProject",
                }
                LabeledBox {
                    name: "description",
                    kind: "text",
                    required: false,
                    placeholder: "this is a description",
                }
                section { class: "button-wrapper",
                    button { r#type: "submit", NewIcon {} }
                }
            }
        }
    }
}
