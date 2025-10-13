use crate::prelude::*;
use dioxus::desktop::{muda, tao};
use muda::Menu;
use tao::window::WindowBuilder;

pub fn launch() {
    let builder = WindowBuilder::new()
        .with_title("SimpleAi - (dev)")
        .with_decorations(false);

    dioxus::desktop::launch::launch_virtual_dom(
        VirtualDom::new(App),
        dioxus::desktop::Config::new()
            .with_window(builder)
            .with_menu(Menu::new()),
    );
}
