use crate::{Player, Turn};
use implicit_clone::{unsync::IString, ImplicitClone};
use jailbird_choice::Choice;

pub(crate) type NativeFunction = fn(Context) -> Choice;

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

impl Function {
    #[cfg(feature = "js")]
    pub fn js_code(&self) -> IString {
        match &self.0 {
            FunctionInner::Native(_) => "[native code]".into(),
            FunctionInner::NativeWithJsExample { example, .. } => (*example).into(),
            FunctionInner::Js(func) => func.body(),
        }
    }
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct Context {
    pub turn: Turn,
    pub this_player: Player,
    pub other_player: Player,
}

#[cfg(feature = "js")]
impl jailbird_js::Context for Context {
    type Turn = Turn;
    type Player = Player;

    fn turn(&self) -> Self::Turn {
        self.turn.clone()
    }

    fn this_player(&self) -> Self::Player {
        self.this_player.clone()
    }

    fn other_player(&self) -> Self::Player {
        self.other_player.clone()
    }
}
