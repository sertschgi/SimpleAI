use super::utils::*;
use tokio::time::*;

#[page]
pub fn Editor() -> Element {
    rsx! {
        main {
            onmounted: move |e| async move {
                sleep(Duration::from_millis(100)).await;
                dioxus::document::eval(
                    r#"
                                    window.activeOnnxViewport = new window.Viewport(document.getElementById("viewport"));
                                "#,
                );
            },
            section { id: "viewport" }
            aside { Search {} }
            document::Script { src: asset!("/assets/scripts/onnx-viewport.js") }
        }
    }
}
