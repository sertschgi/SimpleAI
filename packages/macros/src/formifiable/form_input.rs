use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, parse_str, Expr, LitBool, LitStr};

use super::field_attrs::kind::FormKindOpt;
use super::parsed_field::ParsedField;

pub struct FormInput {
    pub field: ParsedField,
    pub value: Expr,
}

impl ToTokens for FormInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { field, value } = self;

        let mut directory = false;
        let field_name = &field.name;
        let input_type = match &field.field_states.kind {
            FormKindOpt::File => "file",
            FormKindOpt::Dir => {
                directory = true;
                "file"
            }
            _ => "text",
        };

        quote! {
            section {
                class: "FormifyBox",
                h1 {
                    class: "FormifyText",
                    #field_name
                }
                input {
                    class: "FormifyInput",
                    name: #field_name,
                    required: true,
                    value: #value.clone(),
                    r#type: #input_type,
                    "webkitdirectory": #directory,
                }
            }
        }
        .to_tokens(tokens);
    }
}
