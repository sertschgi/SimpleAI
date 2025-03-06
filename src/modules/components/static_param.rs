use sai_backend::utils::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct InternStaticParam {
    pub param: StrongParam,
}
impl From<StrongParam> for InternStaticParam {
    fn from(param: StrongParam) -> Self {
        let b = Self::builder();
        b.param(param).build()
    }
}

#[sai_macros::element("component")]
pub fn StaticParam(style: String, intern: InternStaticParam) -> Element {
    rsx! {
        style { { style } }
        body {
            class: "Param",
        }
    }
}
