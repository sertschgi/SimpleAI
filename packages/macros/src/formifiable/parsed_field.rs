use super::field_attrs::FieldAttributeStates;
use syn::Field;

#[derive(Debug, Default)]
pub struct ParsedField {
    pub field_states: FieldAttributeStates,
}

impl From<Field> for ParsedField {
    fn from(Field { attrs, .. }: Field) -> Self {
        FieldAttributeStates::from(attrs)
    }
}
