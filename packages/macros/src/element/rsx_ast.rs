// %%% rsx_body.rs %%%
// %% includes %%

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Brace,
    Ident, LitStr, Macro, Path, Token,
};

use dioxus_rsx::CallBody;

// %% main %%
// % Element %
#[derive(Clone)]
pub struct Element {
    pub path: Path,
    pub _brace: Brace,
    pub attrs: Punctuated<Attribute, Token![,]>,
    pub body: TokenStream,
}

impl From<Macro> for Element {
    fn from(mac: Macro) -> Self {
        parse2(mac.tokens).expect("Macro could not be parsed!")
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let path = input.parse()?;
        let _brace = braced!(content in input);
        let mut attrs = Punctuated::new();
        while content.peek(Ident) && !(content.peek(Ident) && content.peek2(Brace)) {
            let attr = content.parse()?;
            attrs.push_value(attr);
            if content.peek(Token![,]) {
                attrs.push_punct(content.parse()?);
            }
        }
        println!("CONTENT {content}");
        let body = content.parse()?;
        println!("SUCCESS");
        Ok(Self {
            path,
            _brace,
            attrs,
            body,
        })
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);

        let attrs = &self.attrs;
        let body = &self.body;
        let braced_group = quote! {
            {
                #attrs
                #body
            }
        };
        tokens.extend(braced_group);
    }
}

// % Attribute %
#[derive(Clone)]
pub struct Attribute {
    pub spread: Option<Token![..]>,
    pub name: Ident,
    pub colon: Option<Token![:]>,
    pub value: Option<LitStr>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("INPUT: {input}");
        let spread: Option<Token![..]> = if input.peek(Token![..]) {
            Some(input.parse()?)
        } else {
            None
        };

        let name: Ident = input.parse()?;

        let colon: Option<Token![:]> = if input.peek(Token![:]) {
            Some(input.parse()?)
        } else {
            None
        };

        let value: Option<LitStr> = if colon.is_some() {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Attribute {
            spread,
            name,
            colon,
            value,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}
