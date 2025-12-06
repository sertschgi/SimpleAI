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
    let mut proj = Project {
        name: "".into(),
        author: "".into(),
        description: "".into(),
    };

    rsx! {
        main {
            {
                proj.rsx_creation_form(|e| {
                    let p: Project = e.parsed_values().unwrap();
                    let _ = create_project(
                        simple_ai_backend::modules::utils::prelude::Project::from_values(
                            p.name.clone(),
                            p.description,
                            p.author,
                            Some(p.name),
                        ),
                        false,
                    );
                    // TODO: Popup that shows potential error messages + opens project / goes back to home / project page
                })
            }
        }
    }
}
