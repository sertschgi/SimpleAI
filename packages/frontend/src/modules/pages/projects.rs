use super::super::components::project::Project;
use dioxus::desktop::{window, LogicalSize};
use simple_ai_backend::modules::projects::query::query_projects;
use simple_ai_backend::modules::utils::prelude::ProjectQueryFilter;
use simple_ai_backend::modules::utils::project as b;

use super::utils::*;

#[derive(Clone, PartialEq)]
pub struct ProjectsContext {
    pub results: Signal<b::ProjectResult<Vec<b::ProjectQueryResult>>>,
}

impl ProjectsContext {
    pub fn new() -> Self {
        Self {
            results: Signal::new(b::Project::query(vec![])),
        }
    }

    pub fn query_results(&mut self, query: String) {
        self.results
            .set(b::Project::query(vec![ProjectQueryFilter::Name(query)]));
    }
    pub fn reload(&mut self) {
        self.results.set(b::Project::query(vec![]));
    }
}

#[cfg(not(target_family = "wasm"))]
fn set_size() {
    use dioxus::desktop::{window, wry::dpi::Pixel, LogicalSize};
    window().set_max_inner_size(Some(LogicalSize::new(720, 1080)));
    window().set_min_inner_size(Some(LogicalSize::new(720, 1080)));
}

#[cfg(target_family = "wasm")]
fn set_size() {}

#[page]
pub fn Projects() -> Element {
    set_size();

    let mut error_msg = use_signal(|| String::new());
    let mut error_open = use_signal(|| false);

    let projects_ctx = use_signal(|| ProjectsContext::new());
    use_context_provider(|| projects_ctx);

    let projects = use_resource(move || async move {
        match (projects_ctx().results)() {
            Ok(results) => {
                rsx! {

                    for project_result in results {
                        Project { project_result }
                    }
                }
            }
            Err(e) => {
                error_msg.set(e.to_string());
                error_open.set(true);
                rsx! {}
            }
        }
    });

    let input = move |e: FormEvent| {
        projects_ctx().query_results(e.value());
    };

    rsx! {
        main {
            MsgPopup { msg: error_msg, open: error_open }
            input {
                oninput: input,
                r#type: "search",
                placeholder: "search",
                id: "search",
            }
            article { class: "projects-wrapper",
                div { class: "projects-view", {projects()} }
            }
        }
    }
}
