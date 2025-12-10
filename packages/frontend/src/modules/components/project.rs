use super::utils::*;
use super::{
    breadcrumbs::Breadcrumbs, delete_popup::DeletePopup, window_decorations::WindowDecorations,
};
use chrono::{DateTime, Utc};

#[item]
pub fn Project(name: String, date: DateTime<Utc>, desc: String) -> Element {
    let mut delete_popup_open = use_signal(|| false);
    // TODO: Make a global signal Active Project with the id:
    // TODO: simpleai_backend::project::delete(id) that in ondelete:
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
                Link { to: Route::Edit {}, class: "edit", SettingsIcon {} }
                button {
                    class: "delete",
                    onclick: move |_| { delete_popup_open.set(true) },
                    TrashIcon {}
                    DeletePopup { ondelete: move || {}, open: delete_popup_open }
                }
            }
        }
    }
}
