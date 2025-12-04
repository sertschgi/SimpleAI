pub mod core {
    use super::super::{
        components::{heading_layout::HeadingLayout, top_nav::TopNavLayout},
        pages::prelude::*,
    };
    use crate::utils::*;

    #[derive(Debug, Clone, Routable, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        #[layout(TopNavLayout)]
            #[route("/")]
            Start {},
            #[nest("/editor")]
                #[route("/")]
                Editor {},
            #[end_nest]
            #[layout(HeadingLayout)]
                #[nest("/projects")]
                    #[route("/")]
                    Projects {},
                    #[route("/projects_nav")]
                    ProjectNav {},
                #[end_nest]
                #[nest("/new")]
                    #[route("/")]
                    New {},
    }
}

pub mod prelude {
    pub use super::core::Route;
}
