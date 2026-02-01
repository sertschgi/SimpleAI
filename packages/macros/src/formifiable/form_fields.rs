use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Ident};

use super::parsed_field::ParsedField;

pub struct FormFields<'a> {
    pub name: &'a str,
    pub fields: Vec<ParsedField>,
}

impl<'a> ToTokens for FormFields<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, fields } = self;

        let ident: Ident = parse_str(name).unwrap();

        quote! {
            #[derive(serde::Deserialize, PartialEq, Clone, Default)]
            pub struct #ident {
                #(#fields),*
            }
            impl dioxus::core::IntoAttributeValue for #ident {
                fn into_value(self) -> dioxus::core::AttributeValue {
                    dioxus::core::AttributeValue::None
                }
            }
        }
        .to_tokens(tokens);
    }
}
