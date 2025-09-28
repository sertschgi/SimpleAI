// %%% component.rs %%%
// this is the extra implementation of element for the ElementKind::Component type
// #[component] instead of #[element(type = ElementKind::Component)]

// %% includes %%
// % intern %
use crate::element::kind::*;
// % extern %
use proc_macro2::TokenStream;
// %% main %%
// % impl %
pub fn macro_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    ElementKind::Item {}.extra_macro_impl(attr, item)
}
