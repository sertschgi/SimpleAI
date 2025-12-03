use super::utils::*;
use tokio::time::*;

#[item]
pub fn Divider(id: String, orientation: char, children: Element) -> Element {
    let script: String = format!(
        r#"console.log("before divider"); window.divider = new window.Divider(document.getElementById("{}"), "{}");"#,
        id.clone(),
        orientation,
    );
    rsx! {
        div {
            class: "Divider",
            id,
            onmounted: move |e| {
                let script = script.clone();
                async move {
                    sleep(Duration::from_millis(100)).await;
                    dioxus::document::eval(&script);
                }
            },
            document::Script { src: asset!("/assets/scripts/divider.js") }
        }
    }
}
