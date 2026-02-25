use super::vnode::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewportSave {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddNodeData {
    pub id: Uuid,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveNodeData {
    pub viewport_save: ViewportSave,
}
