use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{
    parse2, parse_quote, token::Token, Attribute, Ident, ItemStruct, LitStr, Meta, MetaList,
};

enum FormAttrs {
    IgnoreAll,
    IgnoreEdit,
    IgnoreCreate,
    IgnoreNone,
}

// eprintln!(
//     "Please use the correct syntax like form::ignore(..) of either // of either all, edit or create"
// )
impl FormAttrs {
    pub fn from_field_attrs(attrs: Vec<Attribute>) -> Self {
        for attr in attrs {
            match attr.meta {
                Meta::List(MetaList { tokens, .. }) => {
                    let ident = parse2::<Ident>(tokens);
                    match ident {
                        Ok(i) => {
                            return match i.to_string().as_str() {
                                "all" => Self::IgnoreAll,
                                "edit" => Self::IgnoreEdit,
                                "create" => Self::IgnoreCreate,
                                _ => break,
                            }
                        }
                        Err(_) => break,
                    }
                }
                _ => break,
            }
        }
        Self::IgnoreNone
    }
}

impl From<Vec<Attribute>> for FormAttrs {
    fn from(attrs: Vec<Attribute>) -> Self {
        Self::from_field_attrs(attrs)
    }
}

impl Default for FormAttrs {
    fn default() -> Self {
        Self::IgnoreNone
    }
}

pub fn macro_impl(item: TokenStream) -> TokenStream {
    let struct_item: ItemStruct = parse2(item).expect("No valid struct!");
    let struct_ident = struct_item.ident;
    let mut edits = TokenStream::new();
    let mut creates = TokenStream::new();
    let mut form_fields = TokenStream::new();
    for field in struct_item.fields {
        let form_field = quote! { #field , };
        let field_ident = field.ident.unwrap();
        let form_attrs: FormAttrs = field.attrs.into();
        let field_name = LitStr::new(&field_ident.to_string(), Span::call_site());
        let creation = quote! {
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
        };
        let edit = quote! {
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
                    value: {obj.#field_ident.clone()},
                    r#type: "text",
                }
            }
        };
        match form_attrs {
            FormAttrs::IgnoreNone => {
                edit.to_tokens(&mut edits);
                creation.to_tokens(&mut creates);
                form_field.to_tokens(&mut form_fields);
            }
            FormAttrs::IgnoreEdit => {
                creation.to_tokens(&mut creates);
            }
            FormAttrs::IgnoreCreate => {
                form_field.to_tokens(&mut form_fields);
                edit.to_tokens(&mut edits);
            }
            FormAttrs::IgnoreAll => {}
        }
    }
    let struct_name = struct_ident.to_string();
    let creation_form_name = Ident::new(&format!("{struct_name}CreationForm"), Span::call_site());
    let edit_form_name = Ident::new(&format!("{struct_name}EditForm"), Span::call_site());
    let form_fields_name = Ident::new(&format!("{struct_name}FormFields"), Span::call_site());
    let form_fields_struct = quote! {
        #[derive(Serialize, Deserialize)]
        pub struct #form_fields_name {
            #form_fields
        }
    };
    println!(
        "---------------------- FIELDS: {}",
        form_fields_struct.to_token_stream()
    );
    quote! {
        simple_ai_macros::icon! {
            formify_creation: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M13.0001 10.9999L22.0002 10.9997L22.0002 12.9997L13.0001 12.9999L13.0001 21.9998L11.0001 21.9998L11.0001 12.9999L2.00004 13.0001L2 11.0001L11.0001 10.9999L11 2.00025L13 2.00024L13.0001 10.9999Z"></path></svg>,
            formify_edit: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M15.7279 9.57627L14.3137 8.16206L5 17.4758V18.89H6.41421L15.7279 9.57627ZM17.1421 8.16206L18.5563 6.74785L17.1421 5.33363L15.7279 6.74785L17.1421 8.16206ZM7.24264 20.89H3V16.6473L16.435 3.21231C16.8256 2.82179 17.4587 2.82179 17.8492 3.21231L20.6777 6.04074C21.0682 6.43126 21.0682 7.06443 20.6777 7.45495L7.24264 20.89Z"></path></svg>
        }
        use dioxus::prelude::*;

        #form_fields_struct

        #[component]
        pub fn #creation_form_name(oncreate: Callback<(FormEvent, #form_fields_name)>) -> Element {
            rsx! {
                form {
                    class: "FormifyForm FormifyCreation",
                    onsubmit: move |e| { oncreate.call((e.clone(), e.parsed_values().unwrap()))},

                    div {
                        class: "FormifyInputs",
                        #creates
                    }

                    button {
                        class: "FormifyButton",
                        r#type: "submit",
                        FormifyCreationIcon {}
                    }
                }
            }
        }

        #[component]
        pub fn #edit_form_name(obj: #struct_ident, onedit: Callback<FormEvent>) -> Element {
            rsx! {
                    form {
                    class: "FormifyForm FormifyEdit",
                    onsubmit: onedit,

                    div {
                        class: "FormifyInputs",
                        #edits
                    }

                    button {
                        class: "FormifyButton",
                        r#type: "submit",
                        FormifyEditIcon {}
                    }
                }
            }

        }
    }
}
