// %%% icon.rs %%%

// %% includes %%
use change_case::*;
use dioxus_autofmt::write_block_out;
use dioxus_rsx_rosetta::{rsx_from_html, Dom};
use proc_macro2::{Span, TokenStream};
use quote::*;
use rstml::node::{Node, NodeAttribute, NodeElement, NodeName, NodeText};
use std::borrow::Cow;
use std::str::FromStr;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    Ident, Lit, Token,
};

// %% main %%

// % Field %
struct Field {
    member: Ident,
    svg: String,
    _colon: Token![:],
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            member: input.parse()?,
            _colon: input.parse()?,
            svg: {
                let mut value = String::new();
                let mut needs_space = false;
                while !input.is_empty() && !input.peek(Token![,]) {
                    let mut p;
                    if let Ok(ident) = input.parse::<Ident>() {
                        p = ident.to_string();
                        if needs_space {
                            p.insert(0, ' ');
                        }
                        needs_space = true;
                    } else if let Ok(lit) = input.parse::<Lit>() {
                        p = match lit {
                            Lit::Str(s) => format!("\"{}\"", s.value()),
                            Lit::Int(i) => i.to_string(),
                            Lit::Char(c) => format!("'{}'", c.value()),
                            Lit::Bool(b) => b.value().to_string(),
                            _ => String::new(),
                        };
                        needs_space = true;
                    } else {
                        p = input.parse::<proc_macro2::TokenTree>().unwrap().to_string();
                        needs_space = false;
                    }
                    value.push_str(&p);
                }
                value
            },
        })
    }
}

// % FieldValues %
struct FieldValues {
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for FieldValues {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(FieldValues {
            fields: input.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

// % macro implementation %
pub fn macro_impl(item: TokenStream) -> TokenStream {
    let fields: FieldValues = parse2(item).unwrap();
    let mut out: TokenStream = quote! {
        use dioxus::{core::AttributeValue, prelude::*};
    };
    for field in fields.fields {
        let function_name = pascal_case(&format!("{}Icon", field.member));
        let function_ident = Ident::new(&function_name, Span::call_site());
        let svg_dom = Dom::parse(&field.svg).expect("failed parsing dom");
        let svg_body = rsx_from_html(&svg_dom);
        let block = write_block_out(&svg_body).expect("failed writing block");
        let svg = TokenStream::from_str(&block).unwrap();
        let classes = format!("Icon {}", function_name);
        quote! {
            #[component]
            pub fn #function_ident(
                // #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>
            ) -> Element {
                let mut other_classes = String::new();
                // if let Some(pos) = attributes.iter().position(|x| x.name == "class") {
                //     let value = attributes.remove(pos).value;
                //     if let AttributeValue::Text(text) = value {
                //         other_classes = text;
                //     }
                // }
                let class = format!("{}{other_classes}", #classes);
                rsx! {
                    div {
                        class,
                        // ..attributes,
                        #svg
                    }
                }
            }
        }
        .to_tokens(&mut out);
    }
    out
}
