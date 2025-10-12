pub mod core {
    use super::super::pages::prelude::*;
    use crate::{modules::pages::utils::Breadcrumbs, utils::*};

    fn TopNav() -> Element {
        rsx! {
            Breadcrumbs {}
            Outlet::<Route> {}
        }
    }

    #[derive(Debug, Clone, Routable, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        #[layout(TopNav)]
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
