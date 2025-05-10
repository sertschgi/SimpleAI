use crate::prelude::{components::prelude::*, *};

#[component]
pub fn Draggable(
    #[props(default)] ondraggingstart: Callback<PageVector>,
    #[props(default)] ondragging: Callback<PageVector>,
    #[props(default)] ondraggingend: Callback<PageVector>,
    #[props(default)] position_save: Signal<PageVector>,
    #[props(default)] position_handle: Signal<PageVector>,
    #[props(default)] style_position_save: Signal<&'static str>,
    #[props(default = Signal::new("relative"))] style_position_handle: Signal<&'static str>,
    #[props(default)] z_index_handle: Signal<i32>,
    #[props(default)] display_handle: Signal<&'static str>,
    #[props(default)] user_select_handle: Signal<&'static str>,
    #[props(default)] cursor_handle: Signal<&'static str>,
    #[props(default)] pressed: Signal<bool>,
    #[props(default)] mounted_data: Signal<Option<std::rc::Rc<MountedData>>>,
    children: Element,
) -> Element {
    let get_client_rect = move || async move {
        if let Some(data) = mounted_data() {
            if let Ok(rect) = data.get_client_rect().await {
                return rect;
            }
        }
        PixelsRect::default()
    };

    let context: DragContext = use_context();
    let mousedown = move |_| async move {
        style_position_save.set(style_position_handle());
        position_handle.set(get_client_rect().await.origin.to_vector().cast_unit());
        style_position_handle.set("absolute");
        position_save.set(position_handle());
        user_select_handle.set("none");
        pressed.set(true);
        ondraggingstart.call(position_save());
        cursor_handle.set("alias");
    };
    use_resource(move || async move {
        if pressed() {
            let position = position_save() + context.distance();
            if context.dragging() {
                position_handle.set(position);
                ondragging.call(position);
                cursor_handle.set("alias");
            } else {
                let rect = get_client_rect().await;
                ondraggingend.call(
                    *position_handle.peek() + PageVector::new(rect.width(), rect.height()) / 2f64,
                );
                position_handle.set(Vector2D::zero());
                style_position_handle.set(style_position_save());
                user_select_handle.set("unset");
                *pressed.write_silent() = false;
                cursor_handle.set("unset");
            }
        }
    });

    let mounted = move |e: MountedEvent| async move {
        mounted_data.set(Some(e.data()));
    };

    let mouseover = move |_| {
        cursor_handle.set("grab");
    };

    rsx! {
        div {
            class: "Draggable",
            width: "fit-content",
            height: "fit-content",
            top: 0,
            left: 0,
            z_index: z_index_handle,
            display: "{display_handle}",
            position: "{style_position_handle}",
            user_select: "{user_select_handle}",
            cursor: "{cursor_handle}",
            transform: "translate({position_handle.read().x}px, {position_handle.read().y}px)",
            onmousedown: mousedown,
            onmounted: mounted,
            onmouseover: mouseover,
            { children }
        }
    }
}
