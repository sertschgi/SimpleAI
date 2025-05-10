pub mod modules;

pub mod prelude {
    pub use super::modules::*;
    pub(crate) use dioxus::html::geometry::{euclid::Vector2D, *};
    pub(crate) use dioxus::prelude::*;
    pub(crate) type PageVector = Vector2D<f64, PageSpace>;
    pub(crate) use dioxus::logger::tracing::*;
    pub(crate) use global::context::*;
    pub(crate) use simple_ai_macros::{component, page};
}
