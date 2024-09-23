use implicit_clone::{unsync::IString, ImplicitClone};
use jailbird_choice::Choice;

pub(crate) type NativeFunction = fn(Context<'_>) -> Choice;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct Function(pub(crate) FunctionInner);

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub(crate) enum FunctionInner {
    Native(NativeFunction),
    #[cfg(feature = "inter")]
    NativeWithJsExample {
        func: NativeFunction,
        example: &'static str,
    },
    #[cfg(feature = "inter")]
    Inter(jailbird_inter::Function),
}

impl Function {
    #[cfg(feature = "inter")]
    pub fn js(&self) -> IString {
        match &self.0 {
            FunctionInner::Native(_) => "[native code]".into(),
            FunctionInner::NativeWithJsExample { example, .. } => (*example).into(),
            FunctionInner::Inter(func) => func.body(),
        }
    }
}

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub struct Context<'a> {
    pub this_player: &'a [Choice],
    pub other_player: &'a [Choice],
}
