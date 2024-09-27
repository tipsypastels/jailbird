use crate::{Runtime, View};
use implicit_clone::{unsync::IString, ImplicitClone};
use jailbird_choice::Choice;

pub(crate) type NativeFunction = fn(View) -> Choice;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct Function(pub(crate) FunctionInner);

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub(crate) enum FunctionInner {
    Native(NativeFunction),
    #[cfg(feature = "js")]
    NativeWithJsExample {
        func: NativeFunction,
        example: &'static str,
    },
    #[cfg(feature = "js")]
    Js(jailbird_js::Function),
}

use FunctionInner as Fi;

impl Function {
    pub fn call(&self, rt: &mut Runtime, view: View) -> Choice {
        match &self.0 {
            Fi::Native(func) => func(view),
            #[cfg(feature = "js")]
            Fi::NativeWithJsExample { func, .. } => func(view),
            #[cfg(feature = "js")]
            // TODO: Don't unwrap.
            Fi::Js(func) => rt.js.call(func, view).unwrap(),
        }
    }

    #[cfg(feature = "js")]
    pub fn js_code(&self) -> IString {
        match &self.0 {
            Fi::Native(_) => "[native code]".into(),
            Fi::NativeWithJsExample { example, .. } => (*example).into(),
            Fi::Js(func) => func.body(),
        }
    }
}
