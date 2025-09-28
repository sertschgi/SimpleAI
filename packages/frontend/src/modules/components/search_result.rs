// %%% components / search_result.rs %%%

// %% includes %%
use super::draggable::Draggable;
use super::node::NODE_TRANSFERER;
use super::utils::*;

// %% main %%

// % Search Result % //
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
        *NODE_TRANSFERER.write() = Some(node);
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
