use crate::{Player, Runtime, Turn};
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

use FunctionInner as Fi;

impl Function {
    pub fn call(&self, rt: &mut Runtime, ctx: Context) -> Choice {
        match &self.0 {
            Fi::Native(func) => func(ctx),
            #[cfg(feature = "js")]
            Fi::NativeWithJsExample { func, .. } => func(ctx),
            #[cfg(feature = "js")]
            // TODO: Don't unwrap.
            Fi::Js(func) => rt.js.call(func, ctx).unwrap(),
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
