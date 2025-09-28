use dioxus::prelude::*;
use simple_ai_macros::*;
#[allow(non_snake_case)]
pub fn TestItem() -> Element {
    {
        {
            dioxus_core::Element::Ok({
                fn __original_template() -> &'static dioxus_core::internal::HotReloadedTemplate {
                    static __ORIGINAL_TEMPLATE: ::std::sync::OnceLock<
                        dioxus_core::internal::HotReloadedTemplate,
                    > = ::std::sync::OnceLock::new();
                    if __ORIGINAL_TEMPLATE.get().is_none() {
                        _ = __ORIGINAL_TEMPLATE
                            .set(
                                dioxus_core::internal::HotReloadedTemplate::new(
                                    None,
                                    ::alloc::vec::Vec::new(),
                                    ::alloc::vec::Vec::new(),
                                    ::alloc::vec::Vec::new(),
                                    __TEMPLATE_ROOTS,
                                ),
                            );
                    }
                    __ORIGINAL_TEMPLATE.get().unwrap()
                }
                let __template_read = {
                    static __NORMALIZED_FILE: &'static str = {
                        const PATH: &str = ::const_format::pmr::__AssertStr {
                            x: {
                                const ARGS_OSRCTFL4A: ::const_format::__str_methods::ReplaceInput = ::const_format::__str_methods::ReplaceInputConv(
                                        "/home/sert/Documents/local/github_projects/SimpleAI/packages/macros/tests/element/item.rs",
                                        "\\\\",
                                        "/",
                                    )
                                    .conv();
                                {
                                    const OB: &[::const_format::pmr::u8; ARGS_OSRCTFL4A
                                        .replace_length()] = &ARGS_OSRCTFL4A.replace();
                                    const OS: &::const_format::pmr::str = unsafe {
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = OB;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    OS
                                }
                            },
                        }
                            .x;
                        ::const_format::pmr::__AssertStr {
                            x: {
                                const ARGS_OSRCTFL4A: ::const_format::__str_methods::ReplaceInput = ::const_format::__str_methods::ReplaceInputConv(
                                        PATH,
                                        '\\',
                                        "/",
                                    )
                                    .conv();
                                {
                                    const OB: &[::const_format::pmr::u8; ARGS_OSRCTFL4A
                                        .replace_length()] = &ARGS_OSRCTFL4A.replace();
                                    const OS: &::const_format::pmr::str = unsafe {
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = OB;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    OS
                                }
                            },
                        }
                            .x
                    };
                    static __TEMPLATE: GlobalSignal<
                        Option<dioxus_core::internal::HotReloadedTemplate>,
                    > = GlobalSignal::with_location(
                        || None::<dioxus_core::internal::HotReloadedTemplate>,
                        __NORMALIZED_FILE,
                        4u32,
                        1u32,
                        0usize,
                    );
                    dioxus_core::Runtime::current().ok().map(|_| __TEMPLATE.read())
                };
                let __template_read = match __template_read
                    .as_ref()
                    .map(|__template_read| __template_read.as_ref())
                {
                    Some(Some(__template_read)) => &__template_read,
                    _ => __original_template(),
                };
                let mut __dynamic_literal_pool = dioxus_core::internal::DynamicLiteralPool::new(
                    ::alloc::vec::Vec::new(),
                );
                let __dynamic_nodes: [dioxus_core::DynamicNode; 0usize] = [];
                let __dynamic_attributes: [Box<[dioxus_core::Attribute]>; 0usize] = [];
                #[doc(hidden)]
                static __TEMPLATE_ROOTS: &[dioxus_core::TemplateNode] = &[
                    {
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::elements::main::TAG_NAME,
                            namespace: dioxus_elements::main::NAME_SPACE,
                            attrs: &[
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::main::class.0,
                                    namespace: dioxus_elements::main::class.1,
                                    value: "TestItem",
                                },
                            ],
                            children: &[],
                        }
                    },
                ];
                {
                    let mut __dynamic_value_pool = dioxus_core::internal::DynamicValuePool::new(
                        Vec::from(__dynamic_nodes),
                        Vec::from(__dynamic_attributes),
                        __dynamic_literal_pool,
                    );
                    __dynamic_value_pool.render_with(__template_read)
                }
            })
        }
    }
}
#[allow(non_snake_case)]
#[doc(hidden)]
mod TestItem_completions {
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    /// This enum is generated to help autocomplete the braces after the component. It does nothing
    pub enum Component {
        TestItem {},
    }
}
#[allow(unused)]
pub use TestItem_completions::Component::TestItem;
