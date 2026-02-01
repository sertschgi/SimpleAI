use super::utils::{
    formify::frontend_project::{EditForm, EditFormFields},
    *,
};

use simple_ai_backend::modules::utils::project::*;
use uuid::Uuid;

#[cfg(not(target_family = "wasm"))]
fn set_size() {
    use dioxus::desktop::{window, wry::dpi::Pixel, LogicalSize};
    window().set_max_inner_size(Some(LogicalSize::new(720, 720)));
    window().set_min_inner_size(Some(LogicalSize::new(720, 720)));
}

#[cfg(target_family = "wasm")]
fn set_size() {}

#[page]
pub fn Edit(id: Uuid) -> Element {
    set_size();

    let ProjectValues {
        author, desc, path, ..
    } = Project::try_from(id).unwrap().values;

    let values = use_signal(move || EditFormFields { author, desc, path });

    let mut error_popup_open = use_signal(|| false);
    let mut error_popup_msg = use_signal(|| String::new());

    rsx! {
        main {
            EditForm {
                values,
                callback: move |(_, f): (FormEvent, EditFormFields)| {},
            }

            MsgPopup { msg: error_popup_msg, open: error_popup_open }
        }
    }
}
