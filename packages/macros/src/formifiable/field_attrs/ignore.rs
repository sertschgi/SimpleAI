/// all options of the `#[ignore(..)]` directive
pub enum FormIgnoreOpt {
    All,
    Create,
    Edit,
    None,
}

impl syn::parse::Parse for FormIgnoreOpt {
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
