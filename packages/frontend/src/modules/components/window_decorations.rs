use super::utils::*;

#[cfg(not(target_family = "wasm"))]
use dioxus::desktop::window;

#[cfg(not(target_family = "wasm"))]
#[item]
pub fn WindowDecorations() -> Element {
    let get_fullscreen_icon = || {
        if window().is_maximized() {
            return FullscreenIcon();
        } else {
            return FullscreenExitIcon();
        }
    };

    let mut fullscreen_icon = use_signal(|| get_fullscreen_icon());

    rsx! {
        div {
            section {
                if window().is_minimizable() {
                    button { onclick: |_| { window().set_minimized(true) }, MinimizeIcon {} }
                }
                if window().is_maximizable() {
                    button {
                        onclick: move |_| {
                            window().toggle_maximized();
                            fullscreen_icon.set(get_fullscreen_icon());
                        },
                        {fullscreen_icon()}
                    }
                }
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
