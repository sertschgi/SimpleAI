// %%% pages / editor.rs %%%

// %% includes %%
use super::utils::*;
use std::collections::HashMap;

// %% main %%
#[page]
pub fn Editor() -> Element {
    document::eval(r#"console.log("hello world"); window.run_editor(); window.run_hello();"#);
    rsx! {
        main {
            section { id: "viewport",
                canvas { id: "draw" }
                document::Script { src: asset!("/assets/scripts/viewport.js"), defer: true }
            }
        }
    }
}
