use serde::Deserialize;
use simple_ai_backend::modules::{projects::create::create_project, utils::project::*};
use uuid::Uuid;

use super::utils::*;

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

    let pv = Project::try_from_id(id).unwrap().values;

    rsx! {
        main {
            ProjectValuesEditForm {
                obj: pv.clone(),
                onedit: move |e: FormEvent| {
                    let r = simple_ai_backend::modules::projects::create::create_project(
                        simple_ai_backend::modules::utils::prelude::Project::from_values(
                            pv.name.clone(),
                            pv.desc.clone(),
                            pv.author.clone(),
                            Some(pv.name.clone()),
                        ),
                        true,
                    );
                    _ = match r {
                        // error_popup_msg.push(e.into());
                        // error_popup_open.set(true);
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
