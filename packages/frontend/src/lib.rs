// %%% lib.rs %%%

// %% global exports %%
pub mod modules;

// %% global prelude %%
pub mod prelude {
    pub use super::modules::*;
}

// %% global utils %%
pub(crate) mod utils {
    // % dioxus %
    pub use dioxus::html::geometry::{euclid::Vector2D, *};
    pub use dioxus::logger::tracing::*;
    pub use dioxus::prelude::*;
    // % macros %
    pub use simple_ai_macros::*;
    // % icons %
    pub use super::modules::icons::*;
    // % custom types %
    pub type PageVector = Vector2D<f64, PageSpace>;
}
