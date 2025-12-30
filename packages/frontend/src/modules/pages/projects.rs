use super::super::components::project::Project;
use dioxus::desktop::{window, LogicalSize};
use simple_ai_backend::modules::projects::query::query_projects;
use simple_ai_backend::modules::utils::prelude::ProjectQueryFilter;
use simple_ai_backend::modules::utils::project as b;

use super::utils::*;

#[derive(Clone, PartialEq)]
pub struct ProjectsContext {
    pub results: Signal<Result<Vec<b::Project>, b::ProjectError>>,
}

impl ProjectsContext {
    pub fn new() -> Self {
        Self {
            results: Signal::new(b::Project::query_all(vec![])),
        }
    }
    pub fn query_results(&mut self, query: String) {
        self.results
            .set(b::Project::query_all(vec![ProjectQueryFilter::Name(query)]));
    }
    pub fn reload(&mut self) {
        self.results.set(b::Project::query_all(vec![]));
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

    let projects_ctx = use_signal(|| ProjectsContext::new());
    use_context_provider(|| projects_ctx);

    let input = move |e: FormEvent| {
        projects_ctx().query_results(e.value());
    };

    rsx! {
        main {
            input {
                oninput: input,
                r#type: "search",
                placeholder: "search",
                id: "search",
            }
            article { class: "projects-wrapper",
                div { class: "projects-view",
                    match
                    for project in (projects_ctx().results)() {
                        Project { project }
                    }
                }
            }
        }
    }
}
