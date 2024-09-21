#![warn(missing_debug_implementations)]

use self::{engine::Engine, outcomes::Outcomes};
use std::{fmt, marker::PhantomData};

mod binding;
mod context;
mod engine;
mod outcomes;

pub use binding::{Binding, CallError, CallResult};
pub use context::{ChoiceContext, Context, PlayerContext, TurnContext};

pub struct Interpreter<C> {
    engine: Engine,
    outcomes: Outcomes,
    _context: PhantomData<C>,
}

impl<C> Interpreter<C> {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        let outcomes = Outcomes::new(&mut engine);

        Self {
            engine,
            outcomes,
            _context: PhantomData,
        }
    }

    pub fn bind(&mut self, body: &str) -> Binding {
        Binding::new(body, &mut self.engine)
    }

    pub fn call(&mut self, binding: &Binding, ctx: C) -> CallResult<C>
    where
        C: Context,
    {
        self.engine.set_context(ctx, &self.outcomes);
        binding.call::<C>(&self.outcomes, &mut self.engine)
    }
}

impl<C> Default for Interpreter<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> fmt::Debug for Interpreter<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interpreter").finish_non_exhaustive()
    }
}
