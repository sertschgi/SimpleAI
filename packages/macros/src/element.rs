pub mod attrs;
pub mod function;
pub mod handler;
pub mod kind;
pub mod manifest;
pub mod rsx_ast;

use handler::ElementHandler;
use proc_macro2::TokenStream;

pub fn macro_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    ElementHandler::new(attr, item).handle()
}
