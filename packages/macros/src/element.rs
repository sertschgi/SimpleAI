// %%% element.rs %%%

// %% modules %%
pub mod attrs;
pub mod function;
pub mod handler;
pub mod kind;
pub mod manifest;
pub mod rsx_ast;

// %% includes %%
use handler::ElementHandler;
use proc_macro2::TokenStream;

// %% main %%
// % impl %
pub fn macro_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    ElementHandler::new(attr, item).handle()
}
