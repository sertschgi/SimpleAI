use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Fields};

use super::field_attrs::ignore::FormIgnoreOpt;
use super::form_fields::FormFields;
use super::form_icon::FormIcon;
use super::parsed_field::ParsedField;

pub struct FormTemplate {
    pub name: String,
    pub icon: FormIcon,
    pub fields: FormFields,
    pub inputs: FormInputs,
}

impl ToTokens for FormTemplate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            icon,
            fields,
            inputs,
        } = self;
        let fields_ident = fields.ident;
        quote! {
            #icon
            #fields

            #[component]
            pub fn #name(callback: Callback<(FormEvent, #fields_ident)>) -> Element {
                rsx! {
                    form {
                        class: "FormifyForm",
                        onsubmit: move |e| { callback.call((e.clone(), e.parsed_values().unwrap()))},

                        #inputs

                        button {
                            class: "FormifyButton",
                            r#type: "submit",

                            #icon
                        }
                    }
                }
            }
        }.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub enum FormKind {
    Create,
    Edit,
}

impl FormKind {
    pub fn to_form_template(&self, fields: Vec<ParsedField>) -> FormTemplate {
        match self {
            Self::Create => FormTemplate {
                name: "CreateForm",
                icon: parse_quote! {},
                fields: FormFields::from(fields),
                inputs: FormInputs::from(fields),
            },
            Self::Edit => FormTemplate {
                name: "CreateForm",
                icon: parse_quote! {},
                fields: FormFields::from(fields),
                inputs: FormInputs::from(fields),
            },
        }
    }
}

#[derive(Clone, Default)]
pub struct Forms {
    pub kinds: HashMap<FormKind, Vec<ParsedField>>,
}

impl From<Fields> for Forms {
    fn from(fields: Fields) -> Self {
        let mut kinds = HashMap::new();
        for field in fields {
            let parsed_field = ParsedField::from(field);
            match parsed_field.field_states.ignored {
                FormIgnoreOpt::All => {
                    kinds.insert(FormKind::Create, parsed_field);
                    kinds.insert(FormKind::Edit, parsed_field);
                }
                FormIgnoreOpt::Create => {
                    kinds.insert(FormKind::Create, parsed_field);
                }
                FormIgnoreOpt::Edit => {
                    kinds.insert(FormKind::Edit, parsed_field);
                }
                FormIgnoreOpt::None => {}
            }
        }
        Self { kinds }
    }
}

impl ToTokens for Forms {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for kind in self.kinds {
            kind.0.to_form_template(kind.1).to_tokens(tokens);
        }
    }
}
