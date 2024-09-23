#![warn(missing_debug_implementations)]

use self::engine::Engine;
use boa_engine::js_str;
use std::fmt;

mod engine;
mod function;
mod view;

pub use function::{CallError, CallResult, Function};
pub use view::{PlayerView, TurnView, View};

pub struct Js {
    engine: Engine,
}

impl Js {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.init_choice(js_str!("COOPERATE"), true);
        engine.init_choice(js_str!("DEFECT"), false);

        Self { engine }
    }

    pub fn bind(&mut self, body: &str) -> Function {
        Function::new(body, &mut self.engine)
    }

    pub fn call(&mut self, function: &Function, view: impl View) -> CallResult {
        self.engine.set_view(view);
        function.call(&mut self.engine)
    }
}

impl Default for Js {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Js {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Js").finish_non_exhaustive()
    }
}
