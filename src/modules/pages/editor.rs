#[sai_macros::element("page")]
pub fn Editor(style: String, icons: Icons) -> Element {
    use crate::components::prelude::*;

    let mousemove = |e| crate::global::context::DRAG_HANDLER.write().drag(e);
    let mouseup = |e| crate::global::context::DRAG_HANDLER.write().end(e);

    rsx! {
        style { { style } }
        style { "html {{overflow: visible;}}"}
        main {
            onmousemove: mousemove,
            onmouseup: mouseup,
            Divider
            {
                section {
                    Viewport {}
                }
                aside {
                    z_index: 2,
                    nav {}
                    section {
                        Search {}
                    }
                }
            }
        }
    }
}

use crate::platform::Window;
pub fn EditorWindow() -> Window {
    #[cfg(feature = "desktop")]
    {
        Window::new(super::Editor, || {
            use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
            Config::default()
                .with_menu(None)
                .with_disable_drag_drop_handler(true)
                .with_window(
                    WindowBuilder::new()
                        .with_inner_size(LogicalSize::new(1920, 1080))
                        .with_transparent(true),
                )
        })
    }
    #[cfg(feature = "web")]
    {
        Window::new(NAME)
    }
}
