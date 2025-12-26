pub mod core {
    use uuid::Uuid;

    use super::super::{
        components::{
            cursor_animation::CursorAnimationLayout, heading_layout::HeadingLayout,
            popup::PopupEntry, top_nav::TopNavLayout,
        },
        pages::prelude::*,
    };
    use crate::utils::*;

    #[derive(Debug, Clone, Routable, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        #[layout(PopupEntry)]
            #[layout(TopNavLayout)]
                #[layout(CursorAnimationLayout)]
                    #[route("/")]
                    Start {},
                #[end_layout(CursorAnimationLayout)]
                #[nest("/projects")]
                    #[layout(CursorAnimationLayout)]
                        #[layout(HeadingLayout)]
                            #[route("/")]
                            Projects {},
                            #[route("/:id")]
                            ProjectNav { id: Uuid },
                            #[route("/:id/edit")]
                            Edit { id: Uuid },
                        #[end_layout]
                    #[end_layout(CursorAnimationLayout)]
                    #[route("/:id/editor")]
                    Editor { id: Uuid },
                #[end_nest]
                #[layout(CursorAnimationLayout)]
                #[layout(HeadingLayout)]
                    #[nest("/new")]
                            #[route("/")]
                            New {},
    }
}

pub mod prelude {
    pub use super::core::Route;
}
