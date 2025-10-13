pub mod core {
    use super::super::components::top_nav::TopNavLayout;
    use super::super::pages::prelude::*;
    use crate::utils::*;

    #[derive(Debug, Clone, Routable, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        #[layout(TopNavLayout)]
            #[route("/")]
            Start {},
            #[nest("/projects")]
                #[route("/")]
                Projects {},
            #[end_nest]
            #[nest("/new")]
                #[route("/")]
                New {},
    }
}

pub mod prelude {
    pub use super::core::Route;
}
