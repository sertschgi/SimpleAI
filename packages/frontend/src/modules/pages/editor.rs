use super::utils::*;
use dioxus::document::Eval;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use tokio::time::*;
use uuid::Uuid;

const CREATE_VIEWPORT: &str = r#"
    window.activeOnnxViewport = new window.Viewport(dioxus, document.getElementById("viewport"));
"#;

/// # handy for json representation:
/// let dbg: String = ViewportEvent::AddNode(Uuid::default()).into();
/// debug!("{}", dbg);
///
/// ``` { AddNode: ".." } ```
#[derive(Serialize, Deserialize, Debug)]
enum ViewportEvent {
    AddNode(Uuid),
}

impl ViewportEvent {
    pub fn exec(&self, handle: Eval) {
        match self {
            Self::AddNode(id) => {
                // Todo: let node = simple_ai_backend::modules::nodes::query_onnx();
                // let node = json! { { "name": "test"} };
                // handle.send(node);
                // handle.send("hello");
            }
        }
    }
}

impl From<JsonValue> for ViewportEvent {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).expect("Coud not convert ViewportEvent from json")
    }
}

impl Into<String> for ViewportEvent {
    fn into(self) -> String {
        serde_json::to_string(&self).expect("Could not convert ViewportEvent to json.")
    }
}

impl From<String> for ViewportEvent {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Could not convert ViewportEvent from json string.")
    }
}

// TODO:
//  - add a train button
//     -> add something like simple_ai_backend :: onnx :: Project . train()
//  - BUT FIRST: add a save button and
//     -> js getNodesFunction
//     -> convert to Node of backend (serde json)
//     -> add something like simple_ai_backend :: onnx :: Project . save_nodes(Vec<Nodes>)
#[page]
pub fn Editor(children: Element) -> Element {
    rsx! {
        main {
            onmounted: move |e| async move {
                sleep(Duration::from_millis(100)).await;

                let mut handle = document::eval(CREATE_VIEWPORT);

                loop {
                    let s: JsonValue = handle.recv().await.expect("error recieving string");
                    let e: ViewportEvent = s.into();
                    e.exec(handle);
                }

                // loop {
                //     TODO: also convert the string to a rust new Event enum
                //      - the event types of the enum are AddNode and CheckConnection
                //      - the backend will need something like simple_ai_backend :: onnx ::
                //      check_connection (param1, param2)
                //

                //     TODO: If something failes make a notification.
                // debug!("{:?}", viewport_listener.recv::< String > (). await);
                // ViewportEvent::from(viewport_listener.recv::<String>().await.unwrap())
                //     .exec();
                // }
            },
            section { id: "viewport" }
            Divider { id: "editor-aside-viewport", orientation: 'v' }
            aside { Search {} }
            document::Script { src: asset!("/assets/scripts/onnx-viewport.js") }
        }
    }
}
