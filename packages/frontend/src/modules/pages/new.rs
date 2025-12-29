use super::utils::*;
use serde::{Deserialize, Serialize};
use simple_ai_backend::modules::{projects::create::create_project, utils::project::*};

#[cfg(not(target_family = "wasm"))]
fn set_size() {
    use dioxus::desktop::{window, wry::dpi::Pixel, LogicalSize};
    window().set_max_inner_size(Some(LogicalSize::new(720, 720)));
    window().set_min_inner_size(Some(LogicalSize::new(720, 720)));
}

#[cfg(target_family = "wasm")]
fn set_size() {}

#[page]
pub fn New() -> Element {
    set_size();

    let mut error_popup_open = use_signal(|| false);
    let mut error_popup_msg = use_signal(|| String::new());

    rsx! {
        main {
            ProjectValuesCreationForm {
                oncreate: move |(_, f): (FormEvent, ProjectValuesFormFields)| {
                    let r = simple_ai_backend::modules::utils::prelude::Project::new(
                            f.name,
                            f.desc,
                            f.author,
                            f.path,
                        )
                        .create();
                    _ = match r {
                        Ok(Project { id, .. }) => {
                            _ = router().push(Route::ProjectNav { id });
                        }
                        Err(e) => {
                            eprintln!("Error: {e}");
                            error_popup_msg.set(e);
                            error_popup_open.set(true);
                        }
                    };
                },
            
            }

            MsgPopup { msg: error_popup_msg, open: error_popup_open }
        }
    }
}
