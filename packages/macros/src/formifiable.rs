use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{parse2, ItemStruct, LitStr};

pub fn macro_impl(item: TokenStream) -> TokenStream {
    let struct_item: ItemStruct = parse2(item).expect("No valid struct!");
    let struct_ident = struct_item.ident;
    let mut inputs = TokenStream::new();
    for field in struct_item.fields {
        let field_ident = field.ident.unwrap();
        let field_name = LitStr::new(&field_ident.to_string(), Span::call_site());
        quote! {
            section {
                class: "FormifyBox",
                h1 {
                    class: "FormifyText",
                    #field_name
                }
                input {
                    class: "FormifyInput",
                    name: {#field_name},
                    required: true,
                    r#type: "text",
                }
            }
        }
        .to_tokens(&mut inputs);
    }
    quote! {
            use dioxus::prelude::*;
            impl #struct_ident {
                pub fn rsx_form(&mut self) -> Element {
                    rsx! {
                        form {
                             class: "FormifyForm",

                            div {
    class: "FormifyInputs",
                             #inputs
                        }
                            button {
                                class: "FormifyButton",
                                "confirm"
                            }
                        }
                    }
                }
            }
        }
}
