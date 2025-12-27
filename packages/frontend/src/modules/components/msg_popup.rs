use super::popup::Popup;
use super::utils::*;

#[item(no_css = true)]
pub fn MsgPopup(msg: Signal<String>, open: Signal<bool>) -> Element {
    rsx! {
        main {
            if msg() != String::new() {
                Popup { open,
                    p { {msg()} }
                }
            }
        }
    }
}
