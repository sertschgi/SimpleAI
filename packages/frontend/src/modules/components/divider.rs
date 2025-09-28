// %%% components / divider.rs %%%

// %% includes %%
use super::utils::*;

// %% main %%
#[component]
pub fn Divider(children: Element) -> Element {
    let script = r#"
        let l = document.currentScript.parentElement;
        let middleIndex = Math.floor(l.children.length / 2);
        let wrapper = document.createElement("div");
        wrapper.className = "wrapper";
        let div = document.createElement("div");
        div.className = "inner";
        wrapper.appendChild(div);
        l.insertBefore(wrapper, l.children[middleIndex + 1]);
    "#;
    rsx! {
         article {
             class: "Divider",
             script {
                 { script }
             }
             { children }
         }
    }
}
