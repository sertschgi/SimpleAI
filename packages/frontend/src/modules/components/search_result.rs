use simple_ai_backend::modules::utils::node::Node;

use super::utils::*;

#[derive(PartialEq, Props, Clone, Copy)]
pub struct InternSearchResult {
    pub node: Signal<Node>,
}

impl From<Node> for InternSearchResult {
    fn from(node_context: Node) -> Self {
        Self::builder().node(Signal::new(node_context)).build()
    }
}

#[item]
pub fn SearchResult(intern: InternSearchResult) -> Element {
    rsx! {
        article { class: "draggable-node", id: "{intern.node.cloned().id()}",
            h3 {
                span { id: "name", "{intern.node.cloned().name}" }
            }
            p { id: "description", "{intern.node.cloned().description}" }
        }
    }
}
