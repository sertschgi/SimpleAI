use super::utils::*;

#[derive(Clone)]
struct PopupContext {
    pub popup: Element,
}

// impl Default for PopupContext {
//     fn default() -> Self {
//         Self {
//             popup: use_signal(|| rsx! {}),
//         }
//     }
// }

#[item(no_css = true)]
pub fn PopupEntry() -> Element {
    let popup = use_signal(|| PopupContext {
        popup: {
            rsx! {}
        },
    });
    use_context_provider(|| popup);
    rsx! {
        main {
            {popup().popup}
            Outlet::<Route> {}
        }
    }
}

#[item(no_class = true)]
pub fn Popup(
    children: Element,
    #[props(default = use_signal(|| false))] open: Signal<bool>,
) -> Element {
    let popup = use_signal(|| {
        rsx! {
            main { class: "Popup",
                article { class: "banner",
                    header {
                        button {
                            onclick: move |e| {
                                open.set(false);
                                e.stop_propagation();
                            },
                            CloseIcon {}
                        }
                    }
                    article { class: "content", {children} }
                }
            }
        }
    });

    let mut popup_context: Signal<PopupContext> = use_context();

    use_resource(move || async move {
        if open() {
            popup_context.set(PopupContext { popup: popup() });
            debug!("open");
        } else {
            popup_context.set(PopupContext { popup: rsx! {} });
        }
    });

    rsx! {}
}
