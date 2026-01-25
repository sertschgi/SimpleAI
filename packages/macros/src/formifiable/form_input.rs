#[derive(Clone)]
pub enum FormInputKind {
    Edit,
    Create,
}

impl FormInputKind {
    pub fn name(&self) -> &str {
        match self {
            Self::Edit => "Edit",
            Self::Create => "Create",
        }
    }
}

pub struct FormInput {
    form_kind: FormInputKind,
    field: Field,
}

impl ToTokens for FormInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_input_type = "text";
        let field_ident = self.field.ident.clone().unwrap();
        let field_name = LitStr::new(&field_ident.to_string(), Span::call_site());
        let input_type = match &self.field.ty {
            Type::Path(TypePath { path, .. }) if path.is_ident("PathBuf") => "file",
            _ => default_input_type,
        };

        quote! {
            section {
                class: "FormifyBox",
                h1 {
                    class: "FormifyText",
                    #field_name
                }
                input {
                    class: "FormifyInput",
                    name: {#field_name},
                    required: true,
                    // value: {obj.#field_ident.clone()},
                    r#type: {#input_type},
                    "webkitdirectory": true,
                }
            }
        }
        .to_tokens(tokens);
    }
}
