// %%% args.rs %%%
// %% includes %%
// % intern %
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprLit, Lit, LitBool, LitStr, MetaNameValue, Token,
};
// % extern %
use super::kind::ElementKind;
// %% main %%
// % MetaArgs %
struct MetaArgs {
    pub args: Punctuated<MetaNameValue, Token![,]>,
}

impl Parse for MetaArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MetaArgs {
            args: input.parse_terminated(MetaNameValue::parse, Token![,])?,
        })
    }
}

// % ElementArgs %
pub struct ElementAttrs {
    pub kind: ElementKind,
    pub no_css: bool,
    pub no_class: bool,
    pub entry: bool,
}

impl Default for ElementAttrs {
    fn default() -> Self {
        Self {
            kind: ElementKind::Item,
            no_css: false,
            no_class: false,
            entry: false,
        }
    }
}

impl Parse for ElementAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MetaArgs::parse(input)?.into())
    }
}

impl From<MetaArgs> for ElementAttrs {
    fn from(MetaArgs { args }: MetaArgs) -> Self {
        let mut r = Self::default();
        for arg in args {
            match arg.path.get_ident().unwrap().to_string().as_str() {
                "kind" => {
                    r.kind = match arg.value {
                        Expr::Lit(ExprLit{ lit, .. }) => lit.into(),
                        _ => panic!("Please use a literal like 'Component', 'Page' or 'Entry' for the value of 'kind': here --> {:?}", arg.path),
                    }
                }
                "no_css" => {
                    r.no_css = parse_bool("no_css", &arg).unwrap();
                }
                "no_class" => {
                    r.no_class = parse_bool("no_class", &arg).unwrap();
                }
                "entry" => {
                    r.entry = parse_bool("entry", &arg).unwrap();
                }
                _ => {}
            }
        }
        r
    }
}

fn parse_bool<'a>(when: &'a str, arg: &'a MetaNameValue) -> Result<bool, String> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Bool(LitBool { value, .. }),
        ..
    }) = &arg.value
    {
        return Ok(*value);
    }
    Err(format!(
        "Please use a valid boolean for '{}': here --> {:?}",
        when, arg.path
    ))
}
