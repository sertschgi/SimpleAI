use super::utils::*;
use serde::{Deserialize, Serialize};
use simple_ai_backend::modules::projects::create::create_project;

#[derive(Serialize, Deserialize, Formifiable)]
pub struct Project {
    pub name: String,
    pub author: String,
    pub description: String,
}

#[page]
pub fn New() -> Element {
    let mut error_popup_open = use_signal(|| false);
    let mut error_popup_msg = use_signal(|| "");

    let mut proj = Project {
        name: "".into(),
        author: "".into(),
        description: "".into(),
    };

    rsx! {
        main {
            Popup {
                p { {error_popup_msg()} }
            }
            {
                proj.rsx_creation_form(move |e| {
                    let p: Project = e.parsed_values().unwrap();
                    let r = create_project(
                        simple_ai_backend::modules::utils::prelude::Project::from_values(
                            p.name.clone(),
                            p.description,
                            p.author,
                            Some(p.name),
                        ),
                        false,
                    );
                    _ = match r {
                        // error_popup_msg.push(e.into());
                        // error_popup_open.set(true);
                        Ok(project_id) => {
                            _ = router().push(Route::ProjectNav { project_id });
                        }
                        Err(e) => println!("{e}"),
                    };
                })
            }
        
        }
    }
}
