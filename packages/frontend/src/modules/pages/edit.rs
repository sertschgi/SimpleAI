use super::utils::*;

use simple_ai_backend::modules::utils::project::*;
use uuid::Uuid;

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

    let pv = Project::try_from(id).unwrap().values;
    let onedit = move |e: FormEvent| {
        // error_popup_msg.push(e.into());
        // error_popup_open.set(true);
        // _ = match r {
        //     Ok(Project { id, .. }) => {
        //         _ = router().push(Route::ProjectNav { id });
        //     }
        //     Err(e) => println!("{e}"),
        // };
    };

    rsx! {
        main {
            // FrontendProjectEditForm { obj: pv.clone(), onedit }
        }
    }
}
