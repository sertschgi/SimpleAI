use crate::element::element;
use proc_macro2::TokenStream;

pub fn macro_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    element(item, "components")
}
