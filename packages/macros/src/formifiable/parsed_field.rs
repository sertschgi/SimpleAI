use super::field_attrs::FieldAttributeStates;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Field;

#[derive(Debug, Clone)]
pub struct ParsedField {
    pub name: String,
    pub field_states: FieldAttributeStates,
    pub field: Field,
}

impl From<Field> for ParsedField {
    fn from(field: Field) -> Self {
        let Field { attrs, ident, .. } = field.clone();
        Self {
            name: ident.unwrap().to_string(),
            field_states: FieldAttributeStates::from(attrs),
            field: {
                let mut cleaned_field = field.clone();
                cleaned_field.attrs.clear();
                cleaned_field
            },
        }
    }
}

impl ToTokens for ParsedField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.field.to_tokens(tokens);
    }
}
