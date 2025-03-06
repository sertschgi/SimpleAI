pub mod modules;
pub use modules::*;

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("failed to init logger");
    platform::launch();
}
