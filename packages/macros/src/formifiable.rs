use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{
    parse::Parse, parse2, parse_quote, token::Token, Attribute, Field, Fields, FieldsNamed, Ident,
    ItemStruct, LitStr, Meta, MetaList, Type, TypePath,
};

mod field_attrs;
mod form;
mod form_input;

pub fn macro_impl(item: TokenStream) -> TokenStream {
    let struct_item: ItemStruct = parse2(item).expect("No valid struct!");
    let struct_name = Ident::new(
        &change_case::snake_case(&struct_item.ident.to_string()),
        Span::call_site(),
    );

    let forms = form::Form::parse_all(struct_item.fields);

    quote! {
        pub mod formify {
            pub mod #struct_name {
                use std::path::PathBuf;
                use dioxus::prelude::*;

                simple_ai_macros::icon! {
                    create: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M13.0001 10.9999L22.0002 10.9997L22.0002 12.9997L13.0001 12.9999L13.0001 21.9998L11.0001 21.9998L11.0001 12.9999L2.00004 13.0001L2 11.0001L11.0001 10.9999L11 2.00025L13 2.00024L13.0001 10.9999Z"></path></svg>,
                    edit: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M15.7279 9.57627L14.3137 8.16206L5 17.4758V18.89H6.41421L15.7279 9.57627ZM17.1421 8.16206L18.5563 6.74785L17.1421 5.33363L15.7279 6.74785L17.1421 8.16206ZM7.24264 20.89H3V16.6473L16.435 3.21231C16.8256 2.82179 17.4587 2.82179 17.8492 3.21231L20.6777 6.04074C21.0682 6.43126 21.0682 7.06443 20.6777 7.45495L7.24264 20.89Z"></path></svg>
                }

                #(#forms)*
            }
        }
    }
}
