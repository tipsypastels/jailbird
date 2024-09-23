#![warn(missing_debug_implementations)]

use self::engine::Engine;
use boa_engine::js_str;
use std::fmt;

mod context;
mod engine;
mod function;

pub use context::{Context, Player, Turn};
pub use function::{CallError, CallResult, Function};

pub struct Interpreter {
    engine: Engine,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.init_choice(js_str!("COOPERATE"), true);
        engine.init_choice(js_str!("DEFECT"), false);

        Self { engine }
    }

    pub fn bind(&mut self, body: &str) -> Function {
        Function::new(body, &mut self.engine)
    }

    pub fn call<C: Context>(&mut self, function: &Function, ctx: C) -> CallResult {
        self.engine.set_context(ctx);
        function.call(&mut self.engine)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interpreter").finish_non_exhaustive()
    }
}
