// %%% pages / editor.rs %%%

// %% includes %%
use super::utils::*;
use std::collections::HashMap;

// %% main %%
#[page]
pub fn Editor() -> Element {
    let mut section_contents_map = use_signal(HashMap::new);

    rsx! {
        main {
            DragArea {
                Divider {
                    section { Viewport {} }
                    aside { z_index: 2,
                        nav {
                            FocusButton {
                                onfocus: move || {
                                    section_contents_map.write().insert("search", rsx! {
                                        Search {}
                                        "Hello"
                                    });
                                },
                                onunfocus: move || {
                                    section_contents_map.write().remove("search");
                                },
                                SearchIcon {}
                            }
                        }
                        section {
                            for (_ , e) in section_contents_map() {
                                {e}
                            }
                        }
                    }
                }
            }
        }
    }
}
