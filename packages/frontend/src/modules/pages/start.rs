pub use crate::prelude::{components::prelude::*, *};
use dioxus::router::prelude::*;

#[page]
pub fn Start(
    #[props(into)] search_route: NavigationTarget,
    #[props(into)] new_route: NavigationTarget,
    #[props(into)] editor_route: NavigationTarget,
) -> Element {
    rsx! {
        main {
            article {
                section {
                    NavButton { class: "search", to: search_route, div { class: "icon search",
    svg {
        "viewBox": "0 0 24 24",
        xmlns: "http://www.w3.org/2000/svg",
        fill: "currentColor",
        path { d: "M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z" }
    }
                    } }
                    NavButton { class: "new", to: new_route, div { class: "icon new", }
    svg {
        xmlns: "http://www.w3.org/2000/svg",
        "viewBox": "0 0 24 24",
        fill: "currentColor",
        path { d: "M11 11V5H13V11H19V13H13V19H11V13H5V11H11Z" }
    }

                    }
                    NavButton { class: "editor", to: editor_route, div { class: "icon", id: "editor",

    svg {
        "viewBox": "0 0 24 24",
        fill: "currentColor",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M21 6.75736L19 8.75736V4H10V9H5V20H19V17.2426L21 15.2426V21.0082C21 21.556 20.5551 22 20.0066 22H3.9934C3.44476 22 3 21.5501 3 20.9932V8L9.00319 2H19.9978C20.5513 2 21 2.45531 21 2.9918V6.75736ZM21.7782 8.80761L23.1924 10.2218L15.4142 18L13.9979 17.9979L14 16.5858L21.7782 8.80761Z" }
    }

                    } }
                }
            }
        }
    }
}
