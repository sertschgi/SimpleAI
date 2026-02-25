pub mod event;
pub mod save;
pub mod vnode;

use super::utils::*;
use event::ViewportEvent;
use serde_json::Value as JsonValue;
use tokio::time::*;
use uuid::Uuid;

const CREATE_VIEWPORT: &str = r#"
    console.log("making viewport");
    window.activeOnnxViewport = new window.app.vp.Viewport(
      dioxus,
      document.getElementById("viewport"),
    );
    console.log("viewport: ", window.activeOnnxViewport);
"#;

#[cfg(not(target_family = "wasm"))]
fn set_size() {
    use dioxus::desktop::{window, wry::dpi::Pixel, LogicalSize};
    window().set_max_inner_size(Option::<LogicalSize<i32>>::None);
    window().set_min_inner_size(Option::<LogicalSize<i32>>::None);
    window().set_maximized(true);
    use_drop(|| {
        window().set_maximized(false);
    });
}

#[cfg(target_family = "wasm")]
fn set_size() {}

// TODO:
//  - add a train button
//     -> add something like simple_ai_backend :: onnx :: Project . train()
//  - BUT FIRST: add a save button and
//     -> js getNodesFunction
//     -> convert to Node of backend (serde json)
//     -> add something like simple_ai_backend :: onnx :: Project . save_nodes(Vec<Nodes>)
#[page]
pub fn Editor(id: Uuid, children: Element) -> Element {
    set_size();

    let onmounted = move |_| async move {
        sleep(Duration::from_millis(100)).await;

        let mut handle = document::eval(CREATE_VIEWPORT);

        loop {
            let s: JsonValue = handle.recv().await.expect("error recieving string");
            let e: ViewportEvent = s.into();
            e.exec();
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
    };

    rsx! {
        main { onmounted,
            section { id: "viewport" }
            Divider { id: "editor-aside-viewport", orientation: 'v' }
            aside {
                article {}
                Divider { id: "editor-aside-aside", orientation: 'v' }
                article { Search {} }
            }
            document::Script { src: asset!("/assets/scripts/onnx-viewport.js"), defer: true }
        }
    }
}
