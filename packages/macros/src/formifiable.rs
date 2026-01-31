use std::io::Write;

use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{parse2, Ident, ItemStruct};

mod field_attrs;
mod form;
mod form_fields;
mod form_icon;
mod form_input;
mod parsed_field;

pub fn macro_impl(item: TokenStream) -> TokenStream {
    let struct_item: ItemStruct = parse2(item).expect("No valid struct!");
    let struct_name = Ident::new(
        &change_case::snake_case(&struct_item.ident.to_string()),
        Span::call_site(),
    );

    let forms = form::Forms::from(struct_item.fields);

    let r = quote! {
        pub mod formify {
            pub mod #struct_name {
                use std::path::PathBuf;
                use dioxus::prelude::*;

                #forms
            }
        }
    };

    println!("MACRO DEBUG: {}", r.to_token_stream().to_string());
    std::thread::sleep(std::time::Duration::from_millis(500));

    r
}
