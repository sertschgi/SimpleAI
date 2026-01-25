/// all options of the `#[kind(..)]` directive
pub enum FormKindOpt {
    Text,
    Dir,
    File,
}

impl syn::parse::Parse for FormKindOpt {
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
