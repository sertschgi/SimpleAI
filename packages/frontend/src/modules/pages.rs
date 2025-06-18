// %%% pages.rs %%%

// %% exports %%
pub mod editor;
pub mod new;
pub mod search;
pub mod start;

// %% prelude %%
pub mod prelude {
    pub use super::editor::Editor;
    pub use super::new::New;
    pub use super::search::Search;
    pub use super::start::Start;
}

// %% utils %%
pub(crate) mod utils {
    pub use crate::modules::components::prelude::*;
    pub use crate::utils::*;
}
