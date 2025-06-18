mod element;

macro_rules! create_attr_macro {
    ($name: ident) => {
        mod $name;
        #[proc_macro_attribute]
        pub fn $name(
            attr: proc_macro::TokenStream,
            item: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            $name::macro_impl(syn::parse_macro_input!(attr), syn::parse_macro_input!(item)).into()
        }
    };
}

macro_rules! create_function_style_macro {
    ($name: ident) => {
        mod $name;
        #[proc_macro]
        pub fn $name(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $name::macro_impl(syn::parse_macro_input!(item)).into()
        }
    };
}

create_attr_macro!(component);
create_attr_macro!(page);
create_function_style_macro!(icon);
