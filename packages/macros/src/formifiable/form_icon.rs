use dioxus_autofmt::write_block_out;
use dioxus_rsx_rosetta::{rsx_from_html, Dom};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{parse::Parse, parse_str, Ident};

pub struct FormIcon<'a> {
    pub name: &'a str,
    pub svg: &'a str,
}

impl<'a> ToTokens for FormIcon<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, svg } = self;
        let function_ident: Ident = parse_str(name).unwrap();
        let svg_dom = Dom::parse(svg).expect("failed parsing svg dom");
        let svg_body = rsx_from_html(&svg_dom);
        let block = write_block_out(&svg_body).expect("failed writing block");
        let parsed_svg = TokenStream::from_str(&block).unwrap();
        quote! {
            pub fn #function_ident() -> Element {
                rsx! {
                    div {
                        class: "FormIcon",
                        #parsed_svg
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}
