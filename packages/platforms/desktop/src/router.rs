use crate::prelude::{pages::prelude::*, *};

fn StartPage() -> Element {
    rsx! {
        Start {
            search_route: Route::SearchPage {},
            new_route: Route::NewPage {},
            editor_route: Route::EditorPage {},
        }
    }
}

fn SearchPage() -> Element {
    rsx! { Search {} }
}

fn NewPage() -> Element {
    rsx! { New {} }
}

fn EditorPage() -> Element {
    rsx! { Editor {} }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[redirect("/", || Route::StartPage {}) ]
    #[route("/start")]
    StartPage {},
    #[route("/search")]
    SearchPage {},
    #[route("/new")]
    NewPage {},
    #[route("/editor")]
    EditorPage {},
}
