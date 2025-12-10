// %%% components / draggable.rs %%%

// %% includes %%
use super::drag_area::DragContext;
use super::utils::*;

// %% main %%
#[component]
pub fn Draggable(
    #[props(default)] ondraggingstart: Callback<PageVector>,
    #[props(default)] ondragging: Callback<PageVector>,
    #[props(default)] ondraggingend: Callback<PageVector>,
    #[props(default)] height_handle: Signal<String>,
    #[props(default)] width_handle: Signal<String>,
    #[props(default)] dims_handle: Signal<PageVector>,
    #[props(default)] position_save: Signal<PageVector>,
    #[props(default)] position_handle: Signal<PageVector>,
    #[props(default)] style_position_save: Signal<String>,
    #[props(default = Signal::new("unset".into()))] style_position_handle: Signal<String>,
    #[props(default = Signal::new(10))] z_index_handle: Signal<i32>,
    #[props(default)] display_handle: Signal<String>,
    #[props(default)] user_select_handle: Signal<String>,
    #[props(default)] cursor_handle: Signal<String>,
    #[props(default)] pressed: Signal<bool>,
    #[props(default = true)] pop_back: bool,
    children: Element,
) -> Element {
    // using a use_signal as a variable here because first its private and second it just does not work (this was a pain in the ass to find out...)
    let mut mounted_data: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let mut reset_dims = move || {
        height_handle.set("unset".into());
        width_handle.set("unset".into());
    };

    let mut set_dims = move || {
        height_handle.set(format!("{}px", dims_handle().y));
        width_handle.set(format!("{}px", dims_handle().x));
    };

    let mut context: DragContext = use_context();
    let mousedown = move |_| async move {
        let rect = mounted_data().unwrap().get_client_rect().await.unwrap();

        // set a absolute size and position instead of 100% for example
        position_handle.set(rect.origin.to_vector().cast_unit());
        dims_handle.set(rect.size.to_vector().cast_unit());
        set_dims();

        position_save.set(position_handle());
        style_position_save.set(style_position_handle()); // save the original position property of css to reset it later if using pop_back

        // using position absolute here because it has to be removed form document flow
        style_position_handle.set("absolute".into());

        // cosmetic
        context.cursor_handle.set("grabbing".into());
        user_select_handle.set("none".into());

        pressed.set(true);

        // callback with the position save
        ondraggingstart.call(position_save());
    };
    let _ = use_resource(move || async move {
        // let the distance signal of the context be a dependency that drives this hook
        let distance = context.distance();

        // Check if exactly this one element is pressed
        if !*pressed.peek() {
            return;
        };

        // The distance is from the original position so we also have to use this if we want to use
        // display absolute instead of relative
        let position = position_save() + distance;

        // We check if the context is dragging to tell if the user stopped dragging when the cursor
        // is out of bounds for this type too, because it doesn't work to just use the onmouseup on this type.
        if context.dragging() {
            position_handle.set(position);
            ondragging.call(position); // When really dragging provide an event (callback)
        } else {
            let rect = mounted_data().unwrap().get_client_rect().await.unwrap();
            ondraggingend.call(
                *position_handle.peek() + PageVector::new(rect.width(), rect.height()) / 2f64,
            ); // When not dragging provide an event
            if pop_back {
                position_handle.set(Vector2D::zero()); // This is set to zero because it actually
                                                       // refers to the translate property in css
                style_position_handle.set(style_position_save());
                reset_dims();
            };
            context.cursor_handle.set("grab".into());
            pressed.set(false);
        }
    });

    let mounted = move |e: MountedEvent| {
        mounted_data.set(Some(e.data()));
    };

    let mouseenter = move |_| {
        if !context.dragging() {
            context.cursor_handle.set("grab".into());
        }
    };

    let mouseleave = move |_| {
        if !context.dragging() {
            context.reset_cursor();
        }
    };

    rsx! {
        div {
            top: 0,
            left: 0,
            height: height_handle,
            width: width_handle,
            z_index: z_index_handle,
            display: "{display_handle}",
            position: "{style_position_handle}",
            user_select: "{user_select_handle}",
            cursor: "{cursor_handle}",
            transform: "translate({position_handle.read().x}px, {position_handle.read().y}px)",
            onmousedown: mousedown,
            onmounted: mounted,
            onmouseenter: mouseenter,
            onmouseleave: mouseleave,
            {children}
        }
    }
}
