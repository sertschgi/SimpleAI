use super::utils::*;
use serde::{Deserialize, Serialize};
use simple_ai_backend::modules::{projects::create::create_project, utils::project::*};

#[page]
pub fn New() -> Element {
    let mut error_popup_open = use_signal(|| false);
    let mut error_popup_msg = use_signal(|| "");

    rsx! {
        main {
            Popup {
                p { {error_popup_msg()} }
            }
            ProjectValuesCreationForm {
                oncreate: move |(_, f): (FormEvent, ProjectValuesFormFields)| {
                    // error_popup_msg.push(e.into());
                    // error_popup_open.set(true);
                    let r = create_project(
                        simple_ai_backend::modules::utils::prelude::Project::from_values(
                            f.name,
                            f.desc,
                            f.author,
                            None,
                        ),
                        false,
                    );
                    _ = match r {
                        Ok(Project { id, .. }) => {
                            _ = router().push(Route::ProjectNav { id });
                        }
                        Err(e) => println!("{e}"),
                    };
                },
            }
        }
    }
}
