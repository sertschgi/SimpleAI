pub mod modules;

pub mod prelude {
    pub use super::modules::*;
}

pub(crate) mod utils {
    pub use super::modules::icons::*;
    pub use crate::modules::router::prelude::*;
    pub use dioxus::html::geometry::{euclid::Vector2D, *};
    pub use dioxus::logger::tracing::*;
    pub use dioxus::prelude::*;
    pub use simple_ai_macros::*;
    pub type PageVector = Vector2D<f64, PageSpace>;

    use serde::{Deserialize, Serialize};
    use simple_ai_macros::Formifiable;
    use std::path::PathBuf;
    #[derive(Serialize, Deserialize, Clone, PartialEq, Formifiable)]
    pub struct FrontendProject {
        #[kind(text), ignore(create)]
        pub name: String,
        pub author: String,
        pub desc: String,
        pub path: PathBuf,
    }
}
