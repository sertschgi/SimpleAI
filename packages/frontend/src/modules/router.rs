pub mod core {
    use uuid::Uuid;

    use super::super::{
        components::{heading_layout::HeadingLayout, popup::PopupEntry, top_nav::TopNavLayout},
        pages::prelude::*,
    };
    use crate::utils::*;

    #[derive(Debug, Clone, Routable, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        #[layout(PopupEntry)]
        #[layout(TopNavLayout)]
            #[route("/")]
            Start {},
            #[end_nest]
                #[nest("/projects")]
                    #[layout(HeadingLayout)]
                    #[route("/")]
                    Projects {},
                    #[route("/:id")]
                    ProjectNav { id: Uuid },
                    #[route("/:id/edit")]
                    Edit { id: Uuid },
                    #[end_layout]
                    #[route("/:id/editor")]
                    Editor { id: Uuid },
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
