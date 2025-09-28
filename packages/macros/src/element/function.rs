// %%% function.rs %%%
// %% includes %%
// % extern %
use change_case::*;
use syn::{
    parse::{Parse, ParseStream},
    ItemFn, Macro, Stmt,
};
// % intern %
// %% main %%
// % Element Function %
pub struct ElementFunction {
    pub function: ItemFn,
    pub name: String,
    pub element_name: String,
}

impl ElementFunction {
    fn extract_macro_stmt_mut(&mut self) -> &mut Stmt {
        let stmt = self
            .function
            .block
            .stmts
            .last_mut()
            .expect("There are no statements in your function");

        if let Stmt::Macro(_) = stmt {
            stmt
        } else {
            panic!("Couldn't extract the rsx! block from the function")
        }
    }

    pub fn extract_rsx_macro_stmt_mut(&mut self) -> &mut Stmt {
        let stmt = self.extract_macro_stmt_mut();
        if let Stmt::Macro(mac) = stmt {
            if mac.mac.path.segments.last().unwrap().ident != "rsx" {
                panic!("The last statement as a macro is no rsx macro");
            }
        } else {
            panic!();
        }
        stmt
    }

    pub fn extract_rsx_macro_mut(&mut self) -> &mut Macro {
        if let Stmt::Macro(mac) = self.extract_rsx_macro_stmt_mut() {
            &mut mac.mac
        } else {
            panic!()
        }
    }
}

impl Parse for ElementFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let function: ItemFn = ItemFn::parse(input)?;
        let name = function.sig.ident.to_string();
        let element_name = pascal_case(&name);
        Ok(Self {
            function,
            name,
            element_name,
        })
    }
}
