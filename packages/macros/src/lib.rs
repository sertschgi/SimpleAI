mod tests;

mod icon;
#[proc_macro]
pub fn icon(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    icon::macro_impl(syn::parse_macro_input!(item)).into()
}

mod element;
#[proc_macro_attribute]
pub fn element(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    element::macro_impl(syn::parse_macro_input!(attr), syn::parse_macro_input!(item)).into()
}

mod item;
#[proc_macro_attribute]
pub fn item(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item::macro_impl(syn::parse_macro_input!(attr), syn::parse_macro_input!(item)).into()
}

mod page;
#[proc_macro_attribute]
pub fn page(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    page::macro_impl(syn::parse_macro_input!(attr), syn::parse_macro_input!(item)).into()
}

mod entry;
#[proc_macro_attribute]
pub fn entry(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    entry::macro_impl(syn::parse_macro_input!(attr), syn::parse_macro_input!(item)).into()
}

mod formifiable;
#[proc_macro_derive(Formifiable, attributes(omit, kind))]
pub fn formifiable(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    formifiable::macro_impl(syn::parse_macro_input!(item)).into()
}
