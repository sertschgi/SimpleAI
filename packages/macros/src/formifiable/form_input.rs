use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Expr, Ident};

use super::field_attrs::kind::FormKindOpt;
use super::parsed_field::ParsedField;

#[derive(Debug, Clone)]
pub struct FormInputCommonOpts {
    field_name: String,
    value: Expr,
    required: bool,
}

impl ToTokens for FormInputCommonOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FormInputCommonOpts {
            field_name,
            value,
            required,
        } = &self;
        let field_ident: Ident = parse_str(&field_name).unwrap();
        quote! {
            name: #field_name,
            required: #required,
            value: #value().#field_ident.to_string(),
        }
        .to_tokens(tokens);
    }
}

pub struct FormInputText {
    opts: FormInputCommonOpts,
}
impl ToTokens for FormInputText {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { opts } = &self;
        quote! {
            input {
                #opts
                class: "FormifyTextInput",
                r#type: "text",
            }
        }
        .to_tokens(tokens);
    }
}

pub struct FormInputFile {
    pub opts: FormInputCommonOpts,
    pub directory: bool,
}
impl ToTokens for FormInputFile {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { opts, directory } = &self;
        let FormInputCommonOpts {
            value, field_name, ..
        } = opts;
        let text_input = FormInputText { opts: opts.clone() };
        let field_ident: Ident = parse_str(&field_name).unwrap();
        quote! {
            section {
                class: "FormifyFileInput",

                #text_input

                input {
                    required: false,
                    class: "FormifyFileInputButton",
                    r#type: "file",
                    "webkitdirectory": #directory,
                    onchange: move |e| {
                        let mut new_value = parse_values(e.clone());
                        new_value.#field_ident = match e.values().last().unwrap().1.clone() {
                            FormValue::File(Some(data)) => data.path(),
                            _ => PathBuf::default()
                        };
                        #value.set(new_value);
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}

pub enum FormInputKind {
    File(FormInputFile),
    Text(FormInputText),
}

impl ToTokens for FormInputKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::File(f) => f.to_tokens(tokens),
            Self::Text(f) => f.to_tokens(tokens),
        }
    }
}

pub struct FormInput {
    pub field: ParsedField,
    pub value: Expr,
}

impl ToTokens for FormInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { field, value } = self;

        let field_name = &field.name;

        let opts = FormInputCommonOpts {
            field_name: field_name.clone(),
            value: value.clone(),
            required: true,
        };

        let input = match field.field_states.kind {
            FormKindOpt::Text => FormInputKind::Text(FormInputText { opts }),
            FormKindOpt::File => FormInputKind::File(FormInputFile {
                opts,
                directory: false,
            }),
            FormKindOpt::Dir => FormInputKind::File(FormInputFile {
                opts,
                directory: true,
            }),
        };

        quote! {
            section {
                class: "FormifyBox",
                h1 {
                    class: "FormifyText",
                    #field_name
                }
                #input
            }
        }
        .to_tokens(tokens);
    }
}
