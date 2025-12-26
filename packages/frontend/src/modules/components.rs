pub mod breadcrumbs;
// pub mod connection;
pub mod cursor_animation;
pub mod delete_popup;
pub mod divider;
pub mod drag_area;
pub mod draggable;
pub mod focus_button;
pub mod focus_button_array;
pub mod heading_layout;
pub mod labeled_box;
pub mod nav_button;
pub mod navigation_link;
pub mod navigation_page;
// pub mod node;
pub mod popup;
pub mod project;
// pub mod runtime_param;
pub mod search;
pub mod search_result;
pub mod section_toggle;
// pub mod static_param;
pub mod top_nav;
// pub mod viewport;
pub mod window_decorations;

pub mod prelude {
    pub use super::breadcrumbs::*;
    pub use super::cursor_animation::*;
    pub use super::delete_popup::*;
    pub use super::divider::*;
    pub use super::drag_area::*;
    pub use super::draggable::*;
    pub use super::focus_button::*;
    pub use super::focus_button_array::*;
    pub use super::heading_layout::*;
    pub use super::labeled_box::*;
    pub use super::nav_button::*;
    pub use super::navigation_link::*;
    pub use super::navigation_page::*;
    // pub use super::node::*;
    pub use super::popup::*;
    pub use super::project;
    pub use super::search::*;
    pub use super::search_result::*;

    pub use super::top_nav::*;
    // pub use super::viewport::*;
    pub use super::window_decorations::*;
    // pub mod params {
    // pub use super::super::connection::*;
    // pub use super::super::runtime_param::*;
    // pub use super::super::static_param::*;
    // }
}

pub(crate) mod utils {
    pub use crate::utils::*;

    pub fn pj_name_from_str(string: &str) -> String {
        match uuid::Uuid::parse_str(string) {
            Ok(id) => simple_ai_backend::modules::utils::project::Project::try_from_id(id)
                .unwrap()
                .values
                .name
                .to_string(),
            Err(_) => string.to_string(),
        }
    }
}
