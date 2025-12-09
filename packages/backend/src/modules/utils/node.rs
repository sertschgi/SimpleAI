use super::prelude::*;
use derive_builder::Builder;
use onnx_ir::NodeType;
use std::collections::HashMap;
use uuid::Uuid;
// -------------------- NODE KIND -------------------- //
#[derive(Clone, PartialEq)]
pub enum NodeKind {
    Onnx { onnx: OnnxNode },
    Bundled { bundle: NodeContainer },
}
// -------------------- NODE -------------------- //
pub type StrongNode = StrongContext<Node>;
pub type WeakNode = WeakContext<Node>;

#[derive(Builder, Clone, PartialEq)]
pub struct Node {
    #[builder(setter(into))]
    pub name: String,
    pub version: Version,
    pub kind: NodeKind,
    pub description: String,
    pub author: String,
    pub date: Date,
    #[builder(default)]
    pub position: Option<(f64, f64)>,
}

#[derive(Builder, Clone, PartialEq)]
pub struct OnnxNode {
    pub node_type: NodeType,
    pub name: String,
    pub inputs: Vec<StrongParam>,
    pub outputs: Vec<StrongParam>,
}

impl Node {
    pub fn get_full_env(self) -> Environment {
        let mut env = self.version.env;
        if let NodeKind::Bundled { bundle } = self.kind {
            for context in bundle.tree.iter() {
                let node = context.context.try_lock().unwrap().to_owned();
                env = node.get_full_env().merge(&env).unwrap();
            }
        }

        env
    }

    pub fn from_save_node(
        node: SaveNode,
        param_map: Option<&mut HashMap<u128, (StrongParam, Option<u128>)>>,
    ) -> Self {
        let top = param_map.is_none();
        let mut new_map;
        let param_map: &mut HashMap<u128, (StrongParam, Option<u128>)> = match param_map {
            Some(pm) => pm,
            None => {
                new_map = HashMap::new();
                &mut new_map
            }
        };

        let mut binding = NodeBuilder::default();
        let builder = binding
            .name(node.name)
            .description(node.description)
            .author(node.author)
            .version(Version {
                version: node.version.version,
                env: node.version.env,
            })
            .date(node.date);

        let kind = match node.kind {
            SaveNodeKind::Onnx { onnx } => NodeKind::Onnx {
                onnx: OnnxNode::from_save_node(onnx, param_map),
            },
            SaveNodeKind::Bundled { bundle } => {
                let mut node_container = NodeContainer::new();
                for save_node in bundle {
                    node_container.push_context(StrongContext::from(Node::from_save_node(
                        save_node,
                        Some(param_map),
                    )));
                }
                NodeKind::Bundled {
                    bundle: node_container,
                }
            }
        };

        // Second pass: resolve connections
        if top {
            let resolve = param_map.clone();
            for (_, (strong_param, conn)) in resolve {
                if let Some(conn) = conn {
                    if let Some(target) = param_map.get(&conn) {
                        let mut param = strong_param.context.try_lock().unwrap();
                        if let ParamKind::Runtime { connection, .. } = &mut param.kind {
                            *connection = Some(WeakContext::from(target.0.clone()));
                        }
                    }
                }
            }
        }

        builder.kind(kind).build().expect("Failed to build Node")
    }

    pub fn get_params(&self) -> Vec<StrongParam> {
        match &self.kind {
            NodeKind::Onnx { onnx } => [onnx.inputs.clone(), onnx.outputs.clone()].concat(),
            NodeKind::Bundled { bundle } => {
                let mut res = Vec::new();

                for context in &bundle.tree {
                    let node = context.context.try_lock().unwrap().clone();
                    let node_params = node.get_params();

                    res.extend(node_params.into_iter().filter(|p| {
                        let p = p.context.try_lock().unwrap();
                        match &p.kind {
                            ParamKind::Runtime { connection, .. } => {
                                println!("{:?}", connection.is_some());
                                !connection.is_some()
                            }
                            ParamKind::Static { .. } => true,
                        }
                    }));
                }

                res
            }
        }
    }

    pub fn id(&self) -> Uuid {
        Uuid::new_v5(
            &Uuid::nil(),
            format!(
                "{}/{}/{}/{}",
                self.author, self.name, self.version.version, self.description
            )
            .as_bytes(),
        )
    }
}

impl OnnxNode {
    fn from_save_node(
        node: SaveOnnxNode,
        param_map: &mut HashMap<u128, (StrongParam, Option<u128>)>,
    ) -> Self {
        let mut binding = OnnxNodeBuilder::default();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        // First pass: instantiate Params without setting connections
        for save_param in &node.inputs {
            let strong_param = StrongParam::from(Param {
                name: save_param.name.clone(),
                desc: save_param.desc.clone(),
                dtype: save_param.dtype.clone(),
                kind: match &save_param.kind {
                    SaveParamKind::Static { value } => ParamKind::Static {
                        value: value.clone(),
                    },
                    SaveParamKind::Runtime { kind, .. } => ParamKind::Runtime {
                        kind: kind.clone(),
                        connection: None,
                        id: save_param.id,
                    },
                },
            });
            let connected_to = match &save_param.kind {
                SaveParamKind::Runtime { connected_to, .. } => *connected_to,
                SaveParamKind::Static { .. } => None,
            };
            param_map.insert(save_param.id, (strong_param.clone(), connected_to));
            inputs.push(strong_param);
        }
        for save_param in &node.outputs {
            let strong_param = StrongParam::from(Param {
                name: save_param.name.clone(),
                desc: save_param.desc.clone(),
                dtype: save_param.dtype.clone(),
                kind: match &save_param.kind {
                    SaveParamKind::Static { value } => ParamKind::Static {
                        value: value.clone(),
                    },
                    SaveParamKind::Runtime { kind, .. } => ParamKind::Runtime {
                        kind: kind.clone(),
                        connection: None,
                        id: save_param.id,
                    },
                },
            });
            let connected_to = match &save_param.kind {
                SaveParamKind::Runtime { connected_to, .. } => *connected_to,
                SaveParamKind::Static { .. } => None,
            };
            param_map.insert(save_param.id, (strong_param.clone(), connected_to));
            outputs.push(strong_param);
        }

        let builder = binding
            .node_type(node.node_type)
            .name(node.name)
            .inputs(inputs)
            .outputs(outputs);

        builder.build().expect("Failed to build OnnxNode")
    }
}
