use super::utils::*;
use super::{breadcrumbs::Breadcrumbs, window_decorations::WindowDecorations};
use chrono::{DateTime, Utc};

#[item]
pub fn Project(name: String, date: DateTime<Utc>, desc: String) -> Element {
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
                Link { to: Route::ProjectNav {}, class: "open", FolderOpenIcon {} }
                Link { to: Route::Editor {}, class: "edit", SettingsIcon {} }
                Link { to: Route::Editor {}, class: "delete", TrashIcon {} }
            }
        }
    }
}
