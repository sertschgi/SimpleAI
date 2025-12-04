// %%% page.rs %%%
// this is the extra implementation of element for the ElementKind::Component type
// #[page] instead of #[element(type = ElementKind::Page)]

// %% includes %%
// % intern %
use crate::element::kind::*;
// % extern %
use proc_macro2::TokenStream;
// %% main %%
// % impl %
pub fn macro_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    ElementKind::Page {}.extra_macro_impl(attr, item)
}
