// %%% rsx_body.rs %%%
// %% includes %%

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Brace,
    Ident, LitStr, Macro, Path, Token,
};

// %% main %%
// % Element %
#[derive(Clone)]
pub struct Element {
    pub path: Path,
    pub brace: Brace,
    pub attrs: Punctuated<Attribute, Token![,]>,
    pub elmts: Punctuated<Element, Token![,]>,
}

impl From<Macro> for Element {
    fn from(mac: Macro) -> Self {
        parse2(mac.tokens).expect("Macro could not be parsed!")
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            path: input.parse()?,
            brace: braced!(content in input),
            attrs: content.parse_terminated(Attribute::parse, Token![,])?,
            elmts: content.parse_terminated(Element::parse, Token![,])?,
        })
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
    }
}

// % Attribute %
#[derive(Clone)]
pub struct Attribute {
    pub name: Ident,
    pub colon: Token![:],
    pub value: TokenStream,
    pub comma: Option<Token![,]>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            colon: input.parse()?,
            value: input.parse()?,
            comma: input.parse()?,
        })
    }
}
