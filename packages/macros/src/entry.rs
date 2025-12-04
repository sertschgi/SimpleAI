use crate::element::kind::*;
use proc_macro2::TokenStream;
pub fn macro_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    ElementKind::Entry {}.extra_macro_impl(attr, item)
}
