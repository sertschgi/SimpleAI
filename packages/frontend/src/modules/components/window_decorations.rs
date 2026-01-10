use super::utils::*;

#[cfg(not(target_family = "wasm"))]
use dioxus::desktop::window;

#[cfg(not(target_family = "wasm"))]
#[item]
pub fn WindowDecorations() -> Element {
    // let get_fullscreen_icon = || {
    //     if window().is_maximized() {
    //         return FullscreenIcon();
    //     } else {
    //         return FullscreenExitIcon();
    //     }
    // };
    //
    // let toggle_maximized = move || {
    //     debug!("{}", window().is_minimized());
    //     debug!("{}", window().is_maximized());
    //
    //     window().set_fullscreen(true);
    //     // if window().is_maximized() {
    //     //     window().set_maximized(false);
    //     // } else {
    //     //     window().set_maximized(true);
    //     // }
    // };
    //
    // let mut fullscreen_icon = use_signal(|| get_fullscreen_icon());

    rsx! {
        div {
            section {
                if window().is_minimizable() {
                    button { onclick: |_| { window().set_minimized(true) }, MinimizeIcon {} }
                }
                // if window().is_maximizable() {
                //     button {
                //         onclick: move |_| {
                //             toggle_maximized();
                //             fullscreen_icon.set(get_fullscreen_icon());
                //         },
                //         {fullscreen_icon()}
                //     }
                // }
                button { onclick: |_| { window().close() }, CloseIcon {} }
            }
        }
    }
}

#[cfg(target_family = "wasm")]
#[item]
pub fn WindowDecorations() -> Element {
    rsx! {
        div {
            section {
            }
        }
    }
}
