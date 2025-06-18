// %%% pages / editor.rs %%%

// %% includes %%
use super::utils::*;

// %% main %%
#[page]
pub fn Editor() -> Element {
    rsx! {
        style { "html {{overflow: hidden;}} * {{ user_select: none }}" }
        main {
            DragArea {
                Divider
                {
                    section {
                        Viewport {}
                    }
                    aside {
                        z_index: 2,
                        nav {}
                        section {
                            Search {}
                        }
                    }
                }
            }
        }
    }
}
