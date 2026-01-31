pub mod ignore;
pub mod kind;

use ignore::FormIgnoreOpt;
use kind::FormKindOpt;
use syn::{parse2, Attribute, Ident, Meta, MetaList};

pub enum FieldAttributeKind {
    Ignore(FormIgnoreOpt),
    Kind(FormKindOpt),
    None,
}

impl From<Attribute> for FieldAttributeKind {
    fn from(Attribute { meta, .. }: Attribute) -> Self {
        match meta {
            Meta::List(MetaList { tokens, path, .. }) => {
                match path
                    .get_ident()
                    .expect("please provide a valid attribute")
                    .to_string()
                    .as_str()
                {
                    "omit" => Self::Ignore(parse2(tokens.clone()).expect(
                        "please provide a valid omit attribute, options: all, none, edit, create",
                    )),
                    "kind" => {
                        Self::Kind(parse2(tokens.clone()).expect(
                            "please provide a valid kind attribute, options: text, file, dir",
                        ))
                    }
                    _ => Self::None,
                }
            }
            _ => Self::None,
        }
    }
}

pub struct FieldAttribute {
    kind: FieldAttributeKind,
}

impl FieldAttribute {
    pub fn parse_all(attrs: Vec<Attribute>) -> Vec<Self> {
        attrs
            .iter()
            .map(|attr| Self {
                kind: FieldAttributeKind::from(attr.to_owned()),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct FieldAttributeStates {
    pub ignore: FormIgnoreOpt,
    pub kind: FormKindOpt,
}

impl From<Vec<FieldAttribute>> for FieldAttributeStates {
    fn from(attrs: Vec<FieldAttribute>) -> Self {
        let mut fas = Self::default();
        for attr in attrs {
            match attr.kind {
                FieldAttributeKind::Ignore(opt) => fas.ignore = opt,
                FieldAttributeKind::Kind(opt) => fas.kind = opt,
                _ => (),
            }
        }
        fas
    }
}

impl From<Vec<Attribute>> for FieldAttributeStates {
    fn from(attrs: Vec<Attribute>) -> Self {
        Self::from(FieldAttribute::parse_all(attrs))
    }
}
