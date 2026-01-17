use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{
    parse::Parse, parse2, parse_quote, token::Token, Attribute, Field, Fields, FieldsNamed, Ident,
    ItemStruct, LitStr, Meta, MetaList, Type, TypePath,
};

#[derive(Clone)]
pub enum FormKind {
    Edit,
    Create,
}

impl FormKind {
    pub fn name(&self) -> &str {
        match self {
            Self::Edit => "Edit",
            Self::Create => "Create",
        }
    }
}

pub enum FormIgnoreOpt {
    All,
    Create,
    Edit,
    None,
}

impl Parse for FormIgnoreOpt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<Ident>()?.to_string().as_str() {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "create" => Ok(Self::Create),
            "edit" => Ok(Self::Edit),
            _ => Err(syn::Error::new(Span::call_site(), "No valid ignore option")),
        }
    }
}

impl Default for FormIgnoreOpt {
    fn default() -> Self {
        Self::None
    }
}

pub enum FormKindOpt {
    Text,
    Dir,
    File,
}

impl Parse for FormKindOpt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<Ident>()?.to_string().as_str() {
            "text" => Ok(Self::Text),
            "dir" => Ok(Self::Dir),
            "file" => Ok(Self::File),
            _ => Err(syn::Error::new(Span::call_site(), "No valid kind option")),
        }
    }
}

impl Default for FormKindOpt {
    fn default() -> Self {
        Self::Text
    }
}

pub enum FormAttributeKind {
    Ignore(FormIgnoreOpt),
    Kind(FormKindOpt),
    None,
}

impl From<&Meta> for FormAttributeKind {
    fn from(meta: &Meta) -> Self {
        match meta {
            Meta::List(MetaList { tokens, .. }) => match parse2::<Ident>(tokens.clone()) {
                Ok(i) => match i.to_string().as_str() {
                    "ignore" => Self::Ignore(parse2(tokens.clone()).unwrap()),
                    "kind" => Self::Kind(parse2(tokens.clone()).unwrap()),
                    _ => Self::None,
                },
                Err(_) => Self::None,
            },
            _ => Self::None,
        }
    }
}

pub struct FormAttribute {
    kind: FormAttributeKind,
    ident: Ident,
}

impl FormAttribute {
    pub fn parse_all(attrs: Vec<Attribute>) -> Vec<Self> {
        attrs
            .iter()
            .map(|Attribute { meta, .. }| Self {
                kind: meta.into(),
                ident: meta.path().get_ident().unwrap().to_owned(),
            })
            .collect()
    }
}

pub struct FormInput {
    form_kind: FormKind,
    field: Field,
}

impl ToTokens for FormInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_input_type = "text";
        let field_ident = self.field.ident.clone().unwrap();
        let field_name = LitStr::new(&field_ident.to_string(), Span::call_site());
        let input_type = match &self.field.ty {
            Type::Path(TypePath { path, .. }) if path.is_ident("PathBuf") => "file",
            _ => default_input_type,
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
                    name: {#field_name},
                    required: true,
                    // value: {obj.#field_ident.clone()},
                    r#type: {#input_type},
                    "webkitdirectory": true,
                }
            }
        }
        .to_tokens(tokens);
    }
}

pub struct FormFields {
    name: String,
    fields: Vec<Field>,
}

impl FormFields {
    pub fn ident(&self) -> Ident {
        Ident::new(&format!("{}FormFields", self.name), Span::call_site())
    }
}

impl ToTokens for FormFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { fields, .. } = self;

        let mut token_fields = TokenStream::new();
        for field in fields {
            quote! { #field , }.to_tokens(&mut token_fields);
        }

        let form_fields_name = self.ident();
        quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
            pub struct #form_fields_name {
                #token_fields
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct FormTokens {
    pub form_kind: FormKind,
    pub fields: Fields,
}

impl ToTokens for FormTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { form_kind, fields } = self;

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

pub fn macro_impl(item: TokenStream) -> TokenStream {
    let struct_item: ItemStruct = parse2(item).expect("No valid struct!");
    let struct_name = Ident::new(
        &change_case::snake_case(&struct_item.ident.to_string()),
        Span::call_site(),
    );

    let create_form = FormTokens {
        form_kind: FormKind::Create,
        fields: struct_item.fields,
    };
    let edit_form = FormTokens {
        form_kind: FormKind::Edit,
        fields: struct_item.fields,
    };

    println!("{}", create_form.clone().into_token_stream().to_string());

    quote! {
        pub mod formify {
            pub mod #struct_name {
                use std::path::PathBuf;
                use dioxus::prelude::*;

                simple_ai_macros::icon! {
                    create: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M13.0001 10.9999L22.0002 10.9997L22.0002 12.9997L13.0001 12.9999L13.0001 21.9998L11.0001 21.9998L11.0001 12.9999L2.00004 13.0001L2 11.0001L11.0001 10.9999L11 2.00025L13 2.00024L13.0001 10.9999Z"></path></svg>,
                    edit: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M15.7279 9.57627L14.3137 8.16206L5 17.4758V18.89H6.41421L15.7279 9.57627ZM17.1421 8.16206L18.5563 6.74785L17.1421 5.33363L15.7279 6.74785L17.1421 8.16206ZM7.24264 20.89H3V16.6473L16.435 3.21231C16.8256 2.82179 17.4587 2.82179 17.8492 3.21231L20.6777 6.04074C21.0682 6.43126 21.0682 7.06443 20.6777 7.45495L7.24264 20.89Z"></path></svg>
                }

                #create_form
                #edit_form
            }
        }
    }
}
