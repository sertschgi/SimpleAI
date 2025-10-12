// %%% pages / new.rs %%%

// %% includes %%
use super::utils::*;

// %% main %%
#[page]
pub fn New() -> Element {
    rsx! {
        main {
            form {
                LabeledBox {
                    label { r#for: "name", "node name" }
                    input {
                        id: "name",
                        name: "name",
                        r#type: "text",
                        required: "true",
                        placeholder: "SampleProject",
                    }
                }
                LabeledBox {
                    input {
                        id: "name",
                        name: "name",
                        r#type: "text",
                        required: "true",
                        placeholder: "SampleProject",
                    }
                    label { r#for: "name", "project name" }
                }
                input { r#type: "text" }
                LabeledBox {
                    label { r#for: "name", "project name" }
                    input {
                        id: "name",
                        name: "name",
                        r#type: "text",
                        required: "true",
                        placeholder: "SampleProject",
                    }
                }
                LabeledBox {
                    input {
                        id: "name",
                        name: "name",
                        r#type: "text",
                        required: "true",
                        placeholder: "SampleProject",
                    }
                    label { r#for: "name", "project name" }
                }
                input { r#type: "file" }
                input { r#type: "range" }

                input { r#type: "list", list: "options" }

                datalist { id: "options",
                    option { value: "1", "1" }
                    option { value: "2", "2" }
                    option { value: "3", "3" }
                }

                button { r#type: "submit", "create" }
            }
        }
    }
}
