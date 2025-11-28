// %%% pages / editor.rs %%%

// %% includes %%
use super::utils::*;
use std::collections::HashMap;

// %% main %%
#[page]
pub fn Editor() -> Element {
    let future = use_resource(move || async move {
        // You can create as many eval instances as you want
        let mut eval = document::eval(
            r#"const vp = new window.Viewport(document.getElementById("viewport"));
vp.addNode(new window.VNode(100, 100, "A", [new window.Parameter("output", "outA")]));"#,
        );
    });
    rsx! {
        main {
            section { id: "viewport",
                // canvas { id: "draw" }
                document::Script { src: asset!("/assets/scripts/viewport.js"), defer: true }
            }
        }
    }
}
