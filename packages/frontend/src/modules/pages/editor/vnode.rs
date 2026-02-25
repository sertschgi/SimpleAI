//! ## Why?
//! Because there are different Nodes:
//! 1. The ai-model related nodes (Onnx Node, Bundled)
//! 2. The workflow editor nodes
//!
//! They are visually almost the same.
//! Also it separates the frontend from the backend and makes translation (serde) easy.
use serde::{Deserialize, Serialize};

/// Translation Layer for the pos param to js.
#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Translation Layer for Node to js.
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub pos: Position,
    pub label: String,
    pub params: Vec<NodeParameter>,
}

/// Translation Layer for Parameter to js.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NodeParameter {
    pub r#type: String,
    pub name: String,
}

/// Translation Layer for Connection to js.
#[derive(Serialize, Deserialize, Debug)]

pub struct Connection {
    pub from_node: Node,
    pub from_output: NodeParameter,
    pub to_node: Node,
    pub to_input: NodeParameter,
}
