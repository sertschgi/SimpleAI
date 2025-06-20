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

    let style_asset_path = std::path::PathBuf::from("/assets").join("style");

    let pathbuf_asset_path = style_asset_path
        .join(str_type_name)
        .join(str_lowercase_emt_name + ".css");

    let pathbuf_main_css_path = style_asset_path.join("main.css");

    let asset_path = LitStr::new(
        pathbuf_asset_path.to_str().unwrap_or_default(),
        Span::call_site(),
    );

    let main_css_path = LitStr::new(
        pathbuf_main_css_path.to_str().unwrap_or_default(),
        Span::call_site(),
    );

    let mut main_css_link = quote! {};
    let mut main_css_asset = quote! {};
    if str_type_name == "pages" {
        main_css_link = quote! {
            document::Link { rel: "stylesheet", href: MAIN_CSS },
        };
        main_css_asset = quote! {
            use dioxus::prelude::{asset, manganis, Asset};
            const MAIN_CSS: Asset = asset!(#main_css_path);
        };
    }
    for stmt in &mut out_items.block.stmts {
        if let Stmt::Macro(macro_expr) = stmt {
            if macro_expr.mac.path.is_ident("rsx") {
                let rsx_tokens = macro_expr.mac.tokens.clone();
                macro_expr.mac.tokens = {
                    let mut new_tokens = TokenStream::new();
                    quote! {
                        document::Link { rel: "stylesheet", href: #css_emt_name },
                        #main_css_link
                    }
                    .to_tokens(&mut new_tokens);
                    rsx_tokens.to_tokens(&mut new_tokens);
                    new_tokens
                };
            }
        }
    }

    let out = quote! {
        #main_css_asset
        const #css_emt_name: Asset = asset!(#asset_path);
        #[dioxus::core_macro::component]
        #out_items
    };

    println!("out quote: {}", out.to_string());

    out
}
