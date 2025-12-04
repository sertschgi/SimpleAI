// %%% components / nav_button.rs %%%

// %% includes %%
use super::utils::*;

// %% main %%

#[component]
pub fn FocusButton(
    #[props(default)] children: Element,
    #[props(default)] onfocus: Callback<()>,
    #[props(default)] onunfocus: Callback<()>,
    #[props(default)] onclick: Callback<MouseEvent>,
    #[props(default = use_signal(|| false))] focused: Signal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            "focused": focused(),
            onclick: move |e| {
                if focused() {
                    onunfocus.call(());
                } else {
                    onfocus.call(());
                }
                focused.toggle();
                onclick.call(e);
            },
            ..attributes,
            {children}
        }
    }
}
