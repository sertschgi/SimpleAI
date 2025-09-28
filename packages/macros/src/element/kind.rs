// %%% kind.rs %%%
// %% includes %%
// % intern %
use crate::element::ElementHandler;
// % extern %
use change_case::snake_case;
use proc_macro2::TokenStream;
use std::path::PathBuf;
use syn::Lit;

// %% main %%
// % ElementKind %
#[derive(Clone)]
pub enum ElementKind {
    Item,
    Page,
    Entry,
}

impl ElementKind {
    pub fn kind_name(&self) -> &str {
        match self {
            Self::Item => "item",
            Self::Page => "page",
            Self::Entry => "entry",
        }
    }
    pub fn style_file(&self, emt_name: &str) -> PathBuf {
        match self {
            &Self::Entry => PathBuf::from("entry.css"),
            _ => PathBuf::from(format!("{}s", self.kind_name()))
                .join(format!("{}.css", snake_case(emt_name))),
        }
    }
    pub fn extra_macro_impl(&self, attr: TokenStream, item: TokenStream) -> TokenStream {
        let mut handler = ElementHandler::new(attr, item);
        handler.attrs.kind = self.to_owned();
        handler.handle()
    }
}

impl From<Lit> for ElementKind {
    fn from(lit: Lit) -> Self {
        match lit {
            Lit::Str(s) => match s.value().as_str() {
                "entry" => ElementKind::Entry,
                "page" => ElementKind::Page,
                "component" => ElementKind::Item,
                _ => panic!(
                    "Please provide a valid kind of element: options: 'Component', 'Page' and 'Entry'."
                ),
            },
            _ => panic!("Please provide a valid literal: options: 'Component', 'Page' and 'Entry' "),
        }
    }
}
