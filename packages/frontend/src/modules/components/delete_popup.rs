use super::popup::Popup;
use super::utils::*;
use tokio::time::*;

#[item]
pub fn DeletePopup(
    ondelete: Callback,
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
                            onclick: move |_| async move {
                                open.set(false);
                                sleep(Duration::from_millis(50)).await;
                                ondelete.call(());
                            },
                            AcceptIcon {}
                        }
                    }
                }
            }
        }
    }
}
