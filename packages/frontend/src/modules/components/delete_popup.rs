use super::popup::Popup;
use super::utils::*;

#[item]
pub fn DeletePopup(
    ondelete: fn(),
    #[props(default = use_signal(|| false))] open: Signal<bool>,
) -> Element {
    rsx! {
        main {
            Popup { open,
                article { class: "inner",
                    p { "do you really wanna delete?" }
                    article { class: "options",
                        button { onclick: move |_| { open.set(false) }, CloseIcon {} }
                        button {
                            onclick: move |_| {
                                open.set(false);
                                ondelete()
                            },
                            AcceptIcon {}
                        }
                    }
                }
            }
        }
    }
}
