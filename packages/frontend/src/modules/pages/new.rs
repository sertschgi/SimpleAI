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
                    label { for: "name", "node name" }
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                }
                LabeledBox {
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                    label { for: "name", "project name" }
                }
                input { type: "text" }
                LabeledBox {
                    label { for: "name", "project name" }
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                }
                LabeledBox {
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                    label { for: "name", "project name" }
                }
                input { type: "file" }
                input { type: "range" }

                input { type: "list", list: "options" }

                datalist {
                    id: "options",
                    option { value: "1", "1" }
                    option { value: "2", "2" }
                    option { value: "3", "3" }
                }

                button { type: "submit", "create" }
            }
        }
    }
}
