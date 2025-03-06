use dioxus::html::geometry::{euclid::*, *};
use dioxus::prelude::*;
use sai_backend::utils::prelude::*;
use std::rc::Rc;
#[derive(PartialEq, Props, Clone)]
pub struct InternSearchResult {
    node: StrongNode,
    #[props(default = Signal::default())]
    pub position: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub client_rect: Signal<Rect<f64, Pixels>>,
    #[props(default = Signal::new(String::from("unset")))]
    pub style_pos: Signal<String>,
    #[props(default = Signal::default())]
    pub drag_handle: Signal<crate::global::StrongDragHandle>,
}

impl From<StrongNode> for InternSearchResult {
    fn from(node_context: StrongNode) -> Self {
        Self::builder().node(node_context).build()
    }
}

#[sai_macros::element("component")]
pub fn SearchResult(style: String, icons: Icons, intern: InternSearchResult) -> Element {
    let mousedown = move |e| {
        *crate::global::context::DRAG_NODE.write() = Some(intern.node.clone());
        intern
            .drag_handle
            .set(crate::global::context::DRAG_HANDLER.write().init(
                e,
                intern.client_rect,
                intern.position,
                intern.style_pos,
                true,
            ));
    };

    let mounted = move |e: MountedEvent| async move {
        if let Ok(rect) = e.data().get_client_rect().await {
            intern.client_rect.set(rect);
        }
    };

    rsx! {
        style { {style} }
        div {
            class: "SearchResult",
            onmousedown: mousedown,
            onmounted: mounted,
            top: 0,
            left: 0,
            position: "{intern.style_pos}",
            transform: "translate({(intern.position)().x}px, {(intern.position)().y}px) scale(100%)",
            z_index: 100,
            div {
                class: "wrapper items",
                h3 { span { id: "name", "SampleNode" } }

                div {
                        class: "wrapper",
                    div {
                        class: "wrapper a",
                        a { id: "website", "www.sne.com" }
                    }
                    div {
                        class: "wrapper i",

                        div { id: "open", class: "icon" }
                        div { id: "favorite", class: "icon" }
                    }
                }
            }
            h5 { id: "version", "12.23.1" }
            p { id: "description",
                {
                    "This is a sample description. Don't take it seriously. It describes a node in need of documentation.
          I'll just write more to see how the description looks in html with css..."
                }
            }
            address {
                id: "author",
                "sertgrc"
            }
        }
    }
}
