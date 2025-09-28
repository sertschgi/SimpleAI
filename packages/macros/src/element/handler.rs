// %%% ipl.rs %%%
// this is the implementation of the element macro, which is for handling the style assets and the
// creation of new dioxus components

// %% includes %%
// % extern %
use crate::element::rsx_ast::{Attribute, Attributes, Element};
use dioxus_rsx::{
    AttributeName, AttributeValue, BodyNode, CallBody, Component, DynIdx, HotLiteral,
    HotReloadFormattedSegment, IfmtInput, RsxBlock, RsxItem, Segment, TemplateBody,
};
use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{parse::Parser, parse2, parse_quote, Expr, Ident, LitStr, Macro, Stmt};

// % intern %
use super::{
    attrs::ElementAttrs, function::ElementFunction, kind::ElementKind, manifest::ElementConfig,
};

// %% main %%
pub struct ElementHandler {
    pub attrs: ElementAttrs,
    pub function: ElementFunction,
    pub config: ElementConfig,
}

impl ElementHandler {
    pub fn new(attr: TokenStream, item: TokenStream) -> Self {
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
            self.handle_css(rsx_main_element);
        }
    }

    pub fn handle(&mut self) -> TokenStream {
        self.handle_css();
        self.handle_as_entries();
        self.handle_class();

        let func = &self.function.into_token_stream();
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

        rsx_main_element.elmts.push(parse_quote! {
        document::Link {
            rel: "stylesheet",
            href: asset!(#style_file_lit)
        }});
    }

    fn handle_class(&mut self) {
        if self.attrs.no_class {
            return;
        }
        let elm = {
            if let Some(elm) = rsx_main_element
                .attrs
                .iter_mut()
                .find(|attr| attr.name == "class")
            {
                elm
            } else {
                rsx_main_element.attrs.push(parse_quote! { class: "", });
                rsx_main_element.attrs.last_mut().unwrap()
            }
        };
        let val = parse2::<LitStr>(elm.value.clone()).expect("Couldn't parse attribute value");
        elm.value = LitStr::new(
            &format!("{} Element {}", val.value(), self.function.name),
            Span::call_site(),
        )
        .into_token_stream();
    }
}
