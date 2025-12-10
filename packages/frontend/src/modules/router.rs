pub mod core {
    use super::super::{
        components::{heading_layout::HeadingLayout, top_nav::TopNavLayout},
        pages::prelude::*,
    };
    use crate::utils::*;

    #[derive(Debug, Clone, Routable, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        // #[layout(PopupRoot)]
        #[layout(TopNavLayout)]
            #[route("/")]
            Start {},
            #[end_nest]
                #[nest("/projects")]
                    #[layout(HeadingLayout)]
                    #[route("/")]
                    Projects {},
                    #[route("/projects_nav")]
                    ProjectNav {},
                    #[route("/edit")]
                    Edit {},
                    #[end_layout]
                    #[route("/editor")]
                    Editor {},
                #[end_nest]
                #[layout(HeadingLayout)]
                #[nest("/new")]
                    #[route("/")]
                    New {},
    }
}

pub mod prelude {
    pub use super::core::Route;
}
