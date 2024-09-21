use boa_engine::JsString;
use implicit_clone::ImplicitClone;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct IString16 {
    pub(crate) inner: JsString,
}
