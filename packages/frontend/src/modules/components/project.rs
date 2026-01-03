use super::utils::*;

use super::{delete_popup::*, msg_popup::*};
use crate::modules::pages::projects::ProjectsContext;

use tokio::time::*;
// use chrono::{DateTime, Utc};
use simple_ai_backend::modules::{projects::delete::delete_project, utils::project as b};

#[item]
pub fn Project(project_result: b::ProjectQueryResult) -> Element {
    let project = match project_result {
        Ok(p) => p,
        Err(e) => {
            return rsx! {
                div { class: "Project error",
                    p { "Error while querying projects." }
                    p { {e.to_string()} }
                }
            };
        }
    };

    let ps = Signal::new(project);

    let b::Project {
        id,
        values: b::ProjectValues { name, desc, .. },
    } = { ps() };

    let mut err_popup_open = use_signal(|| false);
    let mut err_popup_msg = use_signal(|| String::new());
    let mut delete_popup_open = use_signal(|| false);

    let projects_ctx = use_context::<Signal<ProjectsContext>>();

    rsx! {
        div {
            MsgPopup { msg: err_popup_msg, open: err_popup_open }
            DeletePopup {
                ondelete: move || async move {
                    match delete_project(ps()) {
                        Err(e) => {
                            eprintln!("Error: {e}, project id: {id}");
                            sleep(Duration::from_millis(50)).await;
                            err_popup_open.set(true);
                            err_popup_msg.set(e);
                        }
                        Ok(()) => {
                            println!("Delted project with id: {id}");
                            projects_ctx().reload();
                        }
                    }
                },
                open: delete_popup_open,
            }
            section { class: "infos",
                h3 { class: "name", {name} }
                // p { class: "date", {date.naive_local().date().to_string()} }
                br { class: "spacer" }
                p { class: "desc", {desc} }
            }
            div { class: "divider" }
            section { class: "actions",
                Link { to: Route::ProjectNav { id }, class: "open", FolderOpenIcon {} }
                Link { to: Route::Edit { id }, class: "edit", SettingsIcon {} }
                button {
                    class: "delete",
                    onclick: move |_| { delete_popup_open.set(true) },
                    TrashIcon {}
                
                }
            }
        }
    }
}
