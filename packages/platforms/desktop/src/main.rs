pub mod platform;

pub mod prelude {
    pub(crate) use super::App;
    pub(crate) use dioxus::prelude::*;
    pub(crate) use simple_ai_frontend::prelude::*;
}

use prelude::*;

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("failed to init logger");
    platform::launch();
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<router::core::Route> {}
    }
}
