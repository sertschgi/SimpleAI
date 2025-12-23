use super::delete_popup::DeletePopup;
use super::utils::*;
use chrono::{DateTime, Utc};
use simple_ai_backend::modules::utils::project as b;

#[item]
pub fn Project(project: b::Project) -> Element {
    let b::ProjectValues { name, desc, .. } = project.values.clone();

    let mut delete_popup_open = use_signal(|| false);

    rsx! {
        div {
            section { class: "infos",
                h3 { class: "name", {name} }
                // p { class: "date", {date.naive_local().date().to_string()} }
                br { class: "spacer" }
                p { class: "desc", {desc} }
            }
            div { class: "divider" }
            section { class: "actions",
                Link {
                    to: Route::ProjectNav {
                        id: project.id,
                    },
                    class: "open",
                    FolderOpenIcon {}
                }
                Link { to: Route::Edit {}, class: "edit", SettingsIcon {} }
                button {
                    class: "delete",
                    onclick: move |_| { delete_popup_open.set(true) },
                    TrashIcon {}
                    DeletePopup {
                        ondelete: move || {
                            println!("{:?}", project);
                        },
                        open: delete_popup_open,
                    }
                }
            }
        }
    }
}
