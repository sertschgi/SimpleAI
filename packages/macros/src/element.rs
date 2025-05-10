use change_case::*;
use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{parse2, parse_quote, Block, Ident, ItemFn, LitStr, Stmt};

pub fn element(item: TokenStream, str_type_name: &str) -> TokenStream {
    let mut out_items: ItemFn = parse2(item).unwrap();

    let fn_name = &out_items.sig.ident;

    let str_emt_name = constant_case(&fn_name.to_string());
    let str_lowercase_emt_name = snake_case(&str_emt_name);
    let str_css_emt_name = str_emt_name + "_CSS";
    let css_emt_name = Ident::new(&str_css_emt_name, Span::call_site());

    let str_asset_path = std::path::PathBuf::from("/assets")
        .join("style")
        .join(str_type_name)
        .join(str_lowercase_emt_name + ".css");

    let asset_path = LitStr::new(
        str_asset_path.to_str().unwrap_or_default(),
        Span::call_site(),
    );

    for stmt in &mut out_items.block.stmts {
        if let Stmt::Expr(syn::Expr::Macro(macro_expr), _) = stmt {
            if macro_expr.mac.path.is_ident("rsx") {
                let rsx_tokens = macro_expr.mac.tokens.clone();

                macro_expr.mac.tokens = {
                    let mut rsx_block: Block =
                        parse2(rsx_tokens).expect("Failed to parse rsx! block");

                    let new_link: Stmt = parse_quote! {
                        document::Link { rel: "stylesheet", href: NAVBAR_CSS },
                    };

                    rsx_block.stmts.insert(0, new_link);

                    rsx_block.to_token_stream()
                };
            }
        }
    }

    quote! {
        const #css_emt_name: Asset = asset!(#asset_path);
        #[dioxus::core_macro::component]
        #out_items
    }
}
