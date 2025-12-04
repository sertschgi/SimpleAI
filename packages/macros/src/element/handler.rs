use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{parse2, parse_quote, LitStr};

use super::{
    attrs::ElementAttrs, function::ElementFunction, kind::ElementKind, manifest::ElementConfig,
};

pub struct ElementHandler {
    pub attrs: ElementAttrs,
    pub function: ElementFunction,
    pub config: ElementConfig,
}

impl ElementHandler {
    pub fn new(attr: TokenStream, item: TokenStream) -> Self {
        println!("-----------------------------------------------------------------------------");
        Self {
            attrs: parse2(attr).expect("Could not parse the function attributes"),
            function: parse2(item).expect("Could not parse the function itself"),
            config: ElementConfig::new(),
        }
    }

    pub fn handle_as_entries(&mut self) {
        match self.attrs.kind {
            ElementKind::Page => {
                if self.config.page_as_entry {
                    self.attrs.entry = true;
                }
            }
            ElementKind::Item => {
                if self.config.component_as_entry {
                    self.attrs.entry = true;
                }
            }
            _ => {}
        }

        if self.attrs.entry {
            self.attrs.kind = ElementKind::Entry;
            self.attrs.no_css = false;
            self.handle_css();
        }
    }

    pub fn handle(&mut self) -> TokenStream {
        self.handle_css();
        self.handle_as_entries();
        self.handle_class();

        let func = &self.function;

        println!("{}", func.into_token_stream());

        quote! {
            #[dioxus::prelude::component]
            #func
        }
    }

    fn handle_css(&mut self) {
        if self.attrs.no_css {
            return;
        }

        let style_file = self
            .config
            .asset_dir
            .join(self.attrs.kind.style_file(&self.function.name));

        let style_file_lit =
            LitStr::new(style_file.to_str().unwrap_or_default(), Span::call_site());

        quote! {
            document::Link {
                rel: "stylesheet",
                href: asset!(#style_file_lit)
            }
        }
        .to_tokens(&mut self.function.macro_ast.body);
    }

    fn handle_class(&mut self) {
        if self.attrs.no_class {
            return;
        }
        println!("BEFORE ELM");
        let elm = {
            if let Some(elm) = self
                .function
                .macro_ast
                .attrs
                .iter_mut()
                .find(|attr| attr.name == "class")
            {
                elm
            } else {
                self.function
                    .macro_ast
                    .attrs
                    .insert(0, parse_quote! { class: "" });
                // if only the class attribute is present there needs to be trailing puncuation
                if self.function.macro_ast.attrs.len() == 1 {
                    self.function.macro_ast.attrs.push_punct(parse_quote! {,});
                }
                self.function.macro_ast.attrs.first_mut().unwrap()
            }
        };
        println!("ELM");
        let previous_value = match elm.value.clone() {
            Some(value) => value.value(),
            None => elm.name.to_string(),
        };
        elm.value = Some(LitStr::new(
            &format!("{} Element {}", previous_value, self.function.name),
            Span::call_site(),
        ));
    }
}
