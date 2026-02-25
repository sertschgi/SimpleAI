use super::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeParamKind {
    Input,
    Output,
}

pub type Connection = WeakContext<Param>;
#[derive(Clone, PartialEq)]
pub enum ParamKind {
    Runtime {
        kind: RuntimeParamKind,
        connection: Option<Connection>,
        id: u128,
    },
    Static {
        value: String,
    },
}

pub type StrongParam = StrongContext<Param>;
pub type WeakParam = WeakContext<Param>;
#[derive(Builder, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub desc: String,
    pub kind: ParamKind,
    pub dtype: DType,
}

impl Param {
    pub fn is_input(&self) -> bool {
        match &self.kind {
            ParamKind::Runtime { kind, .. } => *kind == RuntimeParamKind::Input,
            ParamKind::Static { .. } => true,
        }
    }

    pub fn is_output(&self) -> bool {
        match &self.kind {
            ParamKind::Runtime { kind, .. } => *kind == RuntimeParamKind::Output,
            ParamKind::Static { .. } => false,
        }
    }
}
