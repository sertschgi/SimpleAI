// %%% components / window_decorations.rs %%%
// %% includes %%
use super::utils::*;

// %% main %%
#[item]
pub fn WindowDecorations() -> Element {
    rsx! {
        div {
            section { CloseIcon {} }
        }
    }
}
