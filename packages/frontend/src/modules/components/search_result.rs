use crate::prelude::{components::prelude::Draggable, *};
use simple_ai_backend::utils::prelude::*;

#[derive(PartialEq, Props, Clone, Copy)]
pub struct InternSearchResult {
    pub node: Signal<Node>,
}

impl From<Node> for InternSearchResult {
    fn from(node_context: Node) -> Self {
        Self::builder().node(Signal::new(node_context)).build()
    }
}

#[component]
pub fn SearchResult(intern: InternSearchResult) -> Element {
    let draggingend = move |v: PageVector| {
        let mut node = intern.node.cloned();
        node.position = Some((v.x, v.y));
        *DRAG_NODE.write() = Some(node);
    };

    rsx! {
        Draggable {
            ondraggingend: draggingend,
            div {
                class: "SearchResult",
                div {
                    class: "wrapper items",
                    h3 { span { id: "name", "{intern.node.cloned().name}" } }

                    div {
                            class: "wrapper",
                        div {
                            class: "wrapper i",

                            div { id: "open", class: "icon" }
                        }
                    }
                }
                h5 { id: "version", r#"{intern.node.cloned().date.format("%d/%m/%Y")}"# }
                p {
                    id: "description",
                    "{intern.node.cloned().description}"
                }
                address {
                    id: "author",
                    "{intern.node.cloned().author}"
                }
            }
        }
    }
}
