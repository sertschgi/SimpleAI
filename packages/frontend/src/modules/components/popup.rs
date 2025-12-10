use super::utils::*;

#[item]
pub fn Popup(
    children: Element,
    #[props(default = use_signal(|| false))] open: Signal<bool>,
) -> Element {
    rsx! {
        if open() {
            main {
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
                    {children}
                }
            }
        }
    }
}
