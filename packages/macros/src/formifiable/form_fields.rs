pub struct FormFields {
    name: String,
    fields: Vec<Field>,
}

impl FormFields {
    pub fn ident(&self) -> Ident {
        Ident::new(&format!("{}FormFields", self.name), Span::call_site())
    }
}

impl ToTokens for FormFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { fields, .. } = self;

        let mut token_fields = TokenStream::new();
        for field in fields {
            quote! { #field , }.to_tokens(&mut token_fields);
        }

        let form_fields_name = self.ident();
        quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
            pub struct #form_fields_name {
                #token_fields
            }
        }
        .to_tokens(tokens);
    }
}
