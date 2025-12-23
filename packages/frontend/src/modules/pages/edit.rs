use serde::Deserialize;
use simple_ai_backend::modules::{projects::create::create_project, utils::project as bp};

use super::utils::*;

#[derive(Formifiable, Deserialize)]
pub struct Project {
    pub name: String,
    pub author: String,
    pub description: String,
    pub basenode: String,
}

#[page]
pub fn Edit() -> Element {
    // TODO: Query actual information from caller to edit
    let mut proj = Project {
        name: "".into(),
        author: "".into(),
        description: "".into(),
        basenode: "".into(),
    };

    rsx! {
        main {
            {
                proj.rsx_edit_form(|e| {
                    let p: Project = e.parsed_values().unwrap();
                    let r = simple_ai_backend::modules::projects::create::create_project(
                        simple_ai_backend::modules::utils::prelude::Project::from_values(
                            p.name.clone(),
                            p.description,
                            p.author,
                            Some(p.name),
                        ),
                        true,
                    );
                    _ = match r {
                        // error_popup_msg.push(e.into());
                        // error_popup_open.set(true);
                        Ok(bp::Project { id, .. }) => {
                            _ = router().push(Route::ProjectNav { id });
                        }
                        Err(e) => println!("{e}"),
                    };
                })
            }
        }
    }
}
