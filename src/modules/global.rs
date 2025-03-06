use derive_builder::Builder;
use derive_new::new;
use dioxus::html::geometry::{euclid::*, *};
use dioxus::prelude::*;
use std::{cell::RefCell, rc::*};
#[derive(new, Default)]
pub struct DragHandle {
    obj_pos: Signal<Vector2D<f64, PageSpace>>,
    style_pos: Signal<String>,
}
// impl Default for DragHandle {
//     fn default() -> Self {
//         Self::new(Signal::default(), Signal::default())
//     }
// }

//  TODO: replace this with sai_backend::Context<DragHandle>
pub type StrongDragHandle = Rc<RefCell<DragHandle>>;
pub type WeakDragHandle = Weak<RefCell<DragHandle>>;
pub struct DragHandler {
    cursor_start_pos: Vector2D<f64, PageSpace>,
    obj_start_pos: Vector2D<f64, PageSpace>,
    handles: WeakDragHandle,
    reset: bool,
}
impl DragHandler {
    pub fn new() -> Self {
        Self {
            cursor_start_pos: Vector2D::zero(),
            obj_start_pos: Vector2D::zero(),
            handles: Weak::new(),
            reset: true,
        }
    }
    pub fn clear(&mut self) {
        self.cursor_start_pos = Vector2D::zero();
        self.obj_start_pos = Vector2D::zero();
        self.handles = Weak::new();
        self.reset = true;
    }
    pub fn init(
        &mut self,
        e: MouseEvent,
        client_rect: Signal<Rect<f64, Pixels>>,
        obj_pos: Signal<Vector2D<f64, PageSpace>>,
        style_pos: Signal<String>,
        reset: bool,
    ) -> StrongDragHandle {
        self.cursor_start_pos = e.data.page_coordinates().to_vector();
        self.obj_start_pos = (client_rect)().origin.to_vector().cast_unit();
        self.reset = reset;
        dioxus::logger::tracing::debug!("sp: {:?}, ph: {:?}", self.obj_start_pos, obj_pos());
        let handles_ctx = Rc::new(RefCell::new(DragHandle::new(obj_pos, style_pos)));
        let mut handles = handles_ctx.borrow_mut();
        handles.style_pos.set("fixed".into());
        handles.obj_pos.set(self.obj_start_pos);
        self.handles = Rc::downgrade(&handles_ctx);
        handles_ctx.clone()
    }
    pub fn drag(&mut self, e: MouseEvent) {
        if let Some(handles_ctx) = self.handles.upgrade() {
            let diff = e.data.page_coordinates().to_vector() - self.cursor_start_pos;
            handles_ctx
                .borrow_mut()
                .obj_pos
                .set(self.obj_start_pos + diff);
        }
    }
    pub fn end(&mut self, e: MouseEvent) {
        if let Some(handles_ctx) = self.handles.upgrade() {
            let mut handles = handles_ctx.borrow_mut();
            self.drag(e);
            if self.reset {
                handles.obj_pos.set(Vector2D::zero());
            }
            handles.style_pos.set("unset".into());
        }
        self.clear();
    }
}

pub(crate) mod context {
    use dioxus::signals::*;
    pub(crate) static DRAG_HANDLER: GlobalSignal<super::DragHandler> =
        Signal::global(super::DragHandler::new);
    use sai_backend::utils::prelude::StrongNode;
    pub(crate) static DRAG_NODE: GlobalSignal<Option<StrongNode>> = Signal::global(|| None);
    use crate::components::prelude::params::*;
    pub(crate) static CONNECTION: GlobalSignal<Option<InternConnection>> = Signal::global(|| None);
}
