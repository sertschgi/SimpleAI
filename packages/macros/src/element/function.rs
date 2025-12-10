use crate::element::function;

use super::rsx_ast::Element;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse2, ItemFn, Macro, Stmt, StmtMacro,
};

#[derive(Clone)]
pub enum AstResult {
    Success(Element),
    Failure(TokenStream),
}

impl ToTokens for AstResult {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Success(e) => e.to_tokens(tokens),
            Self::Failure(ts) => ts.to_tokens(tokens),
        }
    }
}

impl From<Macro> for AstResult {
    fn from(mac: Macro) -> Self {
        match parse2(mac.tokens.clone()) {
            Ok(e) => Self::Success(e),
            Err(_) => Self::Failure(mac.tokens),
        }
    }
}

#[derive(Clone)]
pub struct ElementFunction {
    pub function: ItemFn,
    pub name: String,
    pub macro_ast: AstResult,
}

impl ElementFunction {
    fn extract_macro_mut(function: &mut ItemFn) -> syn::Result<&mut Macro> {
        let no_stmt_error = syn::Error::new_spanned(&function, "No last statement in function");
        let not_macro_error = syn::Error::new_spanned(&function, "Last statement is no macro");
        let last = function.block.stmts.last_mut().ok_or(no_stmt_error)?;
        if let Stmt::Macro(StmtMacro { mac, .. }) = last {
            return Ok(mac);
        }
        Err(not_macro_error)
    }
}

impl Parse for ElementFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut function: ItemFn = ItemFn::parse(input)?;
        let name = function.sig.ident.to_string();

        let macro_ast = (*Self::extract_macro_mut(&mut function)?).clone().into();

        Ok(Self {
            function,
            name,
            macro_ast,
        })
    }
}

impl ToTokens for ElementFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut function = self.function.clone();
        let mac = Self::extract_macro_mut(&mut function)
            .expect("Could not convert ElementFunction to tokens");
        mac.tokens = self.macro_ast.clone().into_token_stream();
        function.to_tokens(tokens);
    }
}
