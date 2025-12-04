use super::utils::*;
use tokio::time::*;

const CREATE_VIEWPORT: &str = r#"
    window.activeOnnxViewport = new window.Viewport(document.getElementById("viewport"));
    // window.activeOnnxViewport.listener();
"#;

// TODO:
//  - add a train button
//     -> add something like simple_ai_backend :: onnx :: Project . train()
//  - BUT FIRST: add a save button and
//     -> js getNodesFunction
//     -> convert to Node of backend
//     -> add something like simple_ai_backend :: onnx :: Project . save_nodes(Vec<Nodes>)
#[page]
pub fn Editor() -> Element {
    rsx! {
        main {
            onmounted: move |e| async move {
                sleep(Duration::from_millis(100)).await;
                let mut viewport_listener = document::eval(CREATE_VIEWPORT);
                // loop {
                //     let id: i32 = viewport_listener.recv().await.unwrap();
                //     debug!("id {id}");
                //     TODO: simple_ai_backend :: onnx :: fetch_node_from_id(id) -> Node;
                //      - Then make a function that converts the Node to the js one;
                //      - Lastly document::eval(format!(r#"window.activeOnnxViewport.addNode({})"#, node));
                // }
            },
            section { id: "viewport" }
            Divider { id: "editor-aside-viewport", orientation: 'v' }
            aside { Search {} }
            document::Script { src: asset!("/assets/scripts/onnx-viewport.js") }
        }
    }
}
