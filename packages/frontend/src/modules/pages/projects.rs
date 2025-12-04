use super::super::components::project::Project;
use super::utils::*;
use chrono::{DateTime, Utc};
use dioxus::router::NavigationTarget;

#[page]
pub fn Projects() -> Element {
    rsx! {
        main {
            input { r#type: "search" }
            article { class: "projects-wrapper",
                div { class: "projects-view",
                    Project {
                        name: "sample project",
                        date: Utc::now(),
                        desc: "this is a sample Project for development",
                    }
                    Project {
                        name: "sample project",
                        date: Utc::now(),
                        desc: "this is a sample Project for development",
                    }
                    Project {
                        name: "sample project",
                        date: Utc::now(),
                        desc: "this is a sample Project for development",
                    }
                    Project {
                        name: "sample project",
                        date: Utc::now(),
                        desc: "this is a sample Project for development",
                    }
                }
            }
        }
    }
}
