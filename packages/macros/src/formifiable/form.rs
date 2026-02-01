use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Attribute;
use syn::{parse_quote, parse_str, punctuated::Punctuated, Fields, FnArg, Ident, Token};

use super::field_attrs::ignore::FormIgnoreOpt;
use super::form_fields::FormFields;
use super::form_icon::FormIcon;
use super::form_input::FormInput;
use super::parsed_field::ParsedField;

pub struct FormTemplate<'a> {
    pub name: &'a str,
    pub icon: FormIcon<'a>,
    pub fields: Vec<ParsedField>,
    pub value_attributes: Vec<Attribute>,
}

impl<'a> ToTokens for FormTemplate<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            icon,
            fields,
            value_attributes,
        } = self;
        let form_fields = FormFields {
            name: &format!("{}Fields", &name),
            fields: fields.clone(),
        };
        let form_fields_ident: Ident = parse_str(form_fields.name).unwrap();
        let extra_name: Ident = parse_str("values").unwrap();
        let inputs: Vec<FormInput> = fields
            .clone()
            .iter()
            .map(|field| FormInput {
                field: field.clone(),
                value: parse_quote! { #extra_name },
            })
            .collect();
        let func_ident: Ident = parse_str(name).unwrap();
        let fields_ident: Ident = parse_str(&form_fields.name).unwrap();
        let icon_ident: Ident = parse_str(icon.name).unwrap();

        quote! {
            #icon
            #form_fields

            #[component]
            pub fn #func_ident(callback: Callback<(FormEvent, #fields_ident)>, #(#value_attributes)* #extra_name: Signal<#form_fields_ident>) -> Element {
                fn parse_values(e: FormEvent) -> #form_fields_ident {
                    e.parsed_values().expect("failed to parse values on formify")
                }

                rsx! {
                    form {
                        class: "FormifyForm",
                        onsubmit: move |e| { callback.call((e.clone(), parse_values(e.clone()))); },

                        #(#inputs)*

                        button {
                            class: "FormifyButton",
                            r#type: "submit",

                            #icon_ident {}
                        }
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FormKind {
    Create,
    Edit,
}

impl FormKind {
    pub fn to_form_template(&self, fields: Vec<ParsedField>) -> FormTemplate {
        match self {
            Self::Create => FormTemplate {
                name: "CreateForm",
                icon: FormIcon {
                    name: "CreateFormIcon",
                    svg: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M13.0001 10.9999L22.0002 10.9997L22.0002 12.9997L13.0001 12.9999L13.0001 21.9998L11.0001 21.9998L11.0001 12.9999L2.00004 13.0001L2 11.0001L11.0001 10.9999L11 2.00025L13 2.00024L13.0001 10.9999Z"></path></svg>"#,
                },
                fields,
                value_attributes: parse_quote! { #[props(default = Signal::default())] },
            },
            Self::Edit => FormTemplate {
                name: "EditForm",
                icon: FormIcon {
                    name: "EditFormIcon",
                    svg: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M15.7279 9.57627L14.3137 8.16206L5 17.4758V18.89H6.41421L15.7279 9.57627ZM17.1421 8.16206L18.5563 6.74785L17.1421 5.33363L15.7279 6.74785L17.1421 8.16206ZM7.24264 20.89H3V16.6473L16.435 3.21231C16.8256 2.82179 17.4587 2.82179 17.8492 3.21231L20.6777 6.04074C21.0682 6.43126 21.0682 7.06443 20.6777 7.45495L7.24264 20.89Z"></path></svg>"#,
                },
                fields,
                value_attributes: parse_quote! {},
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
        let mut kinds = HashMap::<FormKind, Vec<ParsedField>>::new();
        kinds.insert(FormKind::Create, Vec::new());
        kinds.insert(FormKind::Edit, Vec::new());
        for field in fields {
            let parsed_field = ParsedField::from(field);
            match parsed_field.field_states.ignore {
                FormIgnoreOpt::None => {
                    kinds
                        .get_mut(&FormKind::Create)
                        .unwrap()
                        .push(parsed_field.clone());
                    kinds
                        .get_mut(&FormKind::Edit)
                        .unwrap()
                        .push(parsed_field.clone());
                }
                FormIgnoreOpt::Edit => {
                    kinds
                        .get_mut(&FormKind::Create)
                        .unwrap()
                        .push(parsed_field.clone());
                }
                FormIgnoreOpt::Create => {
                    kinds
                        .get_mut(&FormKind::Edit)
                        .unwrap()
                        .push(parsed_field.clone());
                }
                FormIgnoreOpt::All => {}
            }
        }
        Self { kinds }
    }
}

impl ToTokens for Forms {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for kind in self.kinds.iter() {
            kind.0.to_form_template(kind.1.to_owned()).to_tokens(tokens);
        }
    }
}
