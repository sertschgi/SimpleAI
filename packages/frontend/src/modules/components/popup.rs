use super::utils::*;

#[item]
pub fn Popup(
    children: Element,
    #[props(default = use_signal(|| false))] open: Signal<bool>,
) -> Element {
    rsx! {
        main {
            if open() {
                article { class: "banner",
                    header {
                        button {
                            onclick: move |e| {
                                debug!("closing popup");
                                open.set(false);
                                e.stop_propagation();
                            },
                            CloseIcon {}
                        }
                    }
                    {children}
                }
            }
        }
    }
}
