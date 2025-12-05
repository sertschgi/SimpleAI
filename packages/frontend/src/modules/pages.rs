// %%% pages.rs %%%

// %% exports %%
pub mod edit;
pub mod editor;
pub mod new;
pub mod project_nav;
pub mod projects;
pub mod search;
pub mod start;

// %% prelude %%
pub mod prelude {
    pub use super::edit::Edit;
    pub use super::editor::Editor;
    pub use super::new::New;
    pub use super::project_nav::ProjectNav;
    pub use super::projects::Projects;
    pub use super::search::Search;
    pub use super::start::Start;
}

// %% utils %%
pub(crate) mod utils {
    pub use crate::modules::components::prelude::*;
    pub use crate::utils::*;
}
