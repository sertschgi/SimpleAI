use crate::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct DragContext {
    pub cursor_start_position: Signal<PageVector>,
    pub dragging: Signal<bool>,
    pub distance: Signal<PageVector>,
}
impl DragContext {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init(&mut self, e: MouseEvent) {
        self.dragging.set(true);
        self.cursor_start_position
            .set(e.page_coordinates().to_vector());
    }
    pub fn moving(&mut self, e: MouseEvent) {
        if self.dragging() {
            self.distance
                .set(e.page_coordinates().to_vector() - self.cursor_start_position.cloned());
        }
    }
    pub fn end(&mut self, e: MouseEvent) {
        self.moving(e);
        self.dragging.set(false);
        self.distance.take();
    }
    pub fn dragging(&self) -> bool {
        self.dragging.cloned()
    }
    pub fn distance(&self) -> PageVector {
        self.distance.cloned()
    }
}

#[component]
pub fn DragArea(children: Element) -> Element {
    let mut context = use_context_provider(DragContext::new);
    let mousedown = move |e: MouseEvent| context.init(e);
    let mousemove = move |e: MouseEvent| context.moving(e);
    let mouseup = move |e: MouseEvent| {
        context.end(e);
    };
    rsx! {
        div {
            class: "DragArea",
            onmousedown: mousedown,
            onmousemove: mousemove,
            onmouseup: mouseup,
            { children }
        }
    }
}
