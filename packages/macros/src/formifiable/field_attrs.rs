pub mod ignore;
pub mod kind;

use ignore::FormIgnoreOpt;
use kind::FormKindOpt;
use syn::{parse2, Attribute, Fields, Ident, Meta, MetaList};

#[derive(Debug, Default)]
pub struct FieldAttributeStates {
    pub ignore: FormIgnoreOpt,
    pub kind: FormKindOpt,
}

impl From<Fields> for FieldAttributeStates {
    fn from(fields: Fields) -> Self {
        fields.iter().collect()
    }
}

impl From<Vec<FieldAttribute>> for FieldAttributeStates {
    fn from(attrs: Vec<FieldAttribute>) -> Self {
        let mut fas = Self::default();
        for attr in attrs {
            match FieldAttributeKind::from(attr) {
                FieldAttributeKind::Ignore(opt) => fas.ignore = opt,
                FieldAttributeKind::Kind(opt) => fas.kind = opt,
                _ => (),
            }
        }
        fas
    }
}

pub enum FieldAttributeKind {
    Ignore(FormIgnoreOpt),
    Kind(FormKindOpt),
    None,
}

impl From<Attribute> for FieldAttributeKind {
    fn from(Attribute { meta, .. }: Attribute) -> Self {
        match meta {
            Meta::List(MetaList { tokens, .. }) => match parse2::<Ident>(tokens.clone()) {
                Ok(i) => match i.to_string().as_str() {
                    "ignore" => Self::Ignore(parse2(tokens.clone()).unwrap()),
                    "kind" => Self::Kind(parse2(tokens.clone()).unwrap()),
                    _ => Self::None,
                },
                Err(_) => Self::None,
            },
            _ => Self::None,
        }
    }
}

pub struct FieldAttribute {
    kind: FieldAttributeKind,
    ident: Ident,
}

impl FieldAttribute {
    pub fn parse_all(attrs: Vec<Attribute>) -> Vec<Self> {
        attrs
            .iter()
            .map(|Attribute { meta, .. }| Self {
                kind: meta.into(),
                ident: meta.path().get_ident().unwrap().to_owned(),
            })
            .collect()
    }
}
