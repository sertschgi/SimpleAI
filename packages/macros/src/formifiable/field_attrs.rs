pub mod ignore;
pub mod kind;

use ignore::FormIgnoreOpt;
use kind::FormKindOpt;

pub struct FieldAttributesStates {
    ignore: FormIgnoreOpt,
    kind: FormKindOpt,
}

impl From<Vec<FieldAttribute>> for FieldAttributeStates {
    fn from(attr: Vec<FieldAttribute>) -> Self {}
}

pub enum FieldAttributeKind {
    Ignore(FormIgnoreOpt),
    Kind(FormKindOpt),
    None,
}

impl From<&Meta> for FieldAttributeKind {
    fn from(meta: &Meta) -> Self {
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
