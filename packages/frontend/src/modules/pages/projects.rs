use super::super::components::project::Project;
use simple_ai_backend::modules::projects::query::query_projects;
use simple_ai_backend::modules::utils::prelude::ProjectQueryFilter;
use simple_ai_backend::modules::utils::project as b;

use super::utils::*;

#[derive(Clone, PartialEq)]
pub struct ProjectsContext {
    pub results: Signal<Vec<b::Project>>,
}

impl ProjectsContext {
    pub fn new() -> Self {
        Self {
            results: Signal::new(query_projects(vec![])),
        }
    }
    pub fn query_results(&mut self, query: String) {
        self.results
            .set(query_projects(vec![ProjectQueryFilter::Name(query)]));
    }
    pub fn reload(&mut self) {
        self.results.set(query_projects(vec![]));
    }
}

#[page]
pub fn Projects() -> Element {
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
                    for project in (projects_ctx().results)() {
                        Project { project }
                    }
                }
            }
        }
    }
}
