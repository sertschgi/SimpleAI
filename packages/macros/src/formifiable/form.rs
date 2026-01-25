use std::collections::HashMap;

use syn::{
    parse::Parse, parse2, parse_quote, token::Token, Attribute, Field, Fields, FieldsNamed, Ident,
    ItemStruct, LitStr, Meta, MetaList, Type, TypePath,
};

use super::field_attrs::FieldAttribute;

pub enum Form {
    Create(Vec<ParsedField>),
    Edit(Vec<ParsedField>),
}

#[derive(Clone, Default)]
pub struct Forms {
    pub create: Vec<ParsedField>,
    pub edit: Vec<ParsedField>,
}

impl Forms {
    pub fn from_fields(fields: Fields) -> Self {
        let forms = Self::default();
        for field in fields {
            let parsed_field = ParsedField::from(field);
            match parsed_field.field_states.ignored {}
        }
    }
}

impl ToTokens for Forms {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { form_kind } = self;

        let filtered: Vec<Field> = fields
            .iter()
            .filter(move |field| {
                let form_attrs: FormAttrs = field.attrs.clone().into();
                match self.form_kind {
                    FormKind::Edit => form_attrs.edit,
                    FormKind::Create => form_attrs.create,
                }
            })
            .cloned()
            .collect();

        let name = form_kind.name().to_string();
        let form_fields_struct = FormFields {
            name: name.clone(),
            fields: filtered.clone(),
        };
        let form_fields_ident = form_fields_struct.ident();
        let form_name = Ident::new(&format!("{name}Form"), Span::call_site());
        let icon = Ident::new(&format!("{name}Icon"), Span::call_site());

        let mut inputs = TokenStream::new();
        for field in filtered {
            FormInput {
                form_kind: form_kind.clone(),
                field: field.clone(),
            }
            .to_tokens(&mut inputs);
        }

        quote! {
            #form_fields_struct

            #[component]
            pub fn #form_name(callback: Callback<(FormEvent, #form_fields_ident)>) -> Element {
                rsx! {
                    form {
                        class: "FormifyForm",
                        onsubmit: move |e| { callback.call((e.clone(), e.parsed_values().unwrap()))},

                        div {
                            class: "FormifyInputs",
                            #inputs
                        }

                        button {
                            class: "FormifyButton",
                            r#type: "submit",
                            #icon {}
                        }
                    }
                }
            }
        }.to_tokens(tokens);
    }
}
