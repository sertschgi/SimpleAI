use super::save::*;
use super::vnode::*;
use dioxus::document;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use simple_ai_backend::modules::{nodes::query::query_nodes, utils::query_filter::NodeQueryFilter};
use uuid::Uuid;

/// Enum for events that can occur in the viewport.
///
/// # Example
/// let dbg: String = ViewportEvent::AddNode(Uuid::default()).into();
/// debug!("{}", dbg);
///
/// ``` { AddNode: ".." } ```
#[derive(Serialize, Deserialize, Debug)]
pub enum ViewportEvent {
    AddNode(AddNodeData),
    Save(SaveNodeData),
    NodeFocused(Uuid),
}

impl ViewportEvent {
    pub fn exec(self) {
        match self {
            Self::AddNode(AddNodeData { id, x, y }) => {
                let node = simple_ai_backend::modules::nodes::query::query_nodes(vec![
                    NodeQueryFilter::Id(id),
                ])
                .tree
                .first()
                .unwrap()
                .context
                .try_lock()
                .unwrap() // TODO: That thing panics if no correct node exists
                .clone();

                let params: Vec<NodeParameter> = node
                    .get_params()
                    .iter()
                    .map(|p| {
                        let param = p.context.try_lock().unwrap();
                        NodeParameter {
                            r#type: if param.is_input() {
                                "input".into()
                            } else {
                                "output".into()
                            },
                            name: param.name.clone(),
                        }
                    })
                    .collect();

                let node = Node {
                    pos: Position { x, y },
                    label: node.name.clone(),
                    params,
                };

                let json_node = serde_json::to_string(&node).unwrap();

                println!("{json_node}");

                document::eval(&format!(
                    r#"window.activeOnnxViewport.handleAddNode({})"#,
                    json_node
                ));
            }
            Self::Save(SaveNodeData { .. }) => {
                // TODO: simple_ai_backend::modules::porject::save_onnx(viewport_save);
                println!("Saving...");
            }
            Self::NodeFocused(id) => {
                // TODO: Fetch the Nodes static params and save the node
                println!("focused...");
            }
        }
    }
}

impl From<JsonValue> for ViewportEvent {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).expect("Could not convert ViewportEvent from json")
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
