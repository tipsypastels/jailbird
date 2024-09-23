#![warn(missing_debug_implementations)]

mod function;
mod player;
mod strategies;
mod strategy;
mod turn;
mod versus;

pub use jailbird_choice::*;
pub use player::Player;
pub use strategy::{Strategy, StrategyBuilder};
pub use turn::Turn;
pub use versus::{Versus, VersusState};

#[derive(Debug)]
pub struct Runtime {
    #[cfg(feature = "js")]
    js: jailbird_js::Js,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "js")]
            js: jailbird_js::Js::new(),
        }
    }

    pub fn strategy<F>(&mut self, f: F) -> Strategy
    where
        F: FnOnce(StrategyBuilder<strategy::Step0>) -> Strategy,
    {
        f(StrategyBuilder::new(self))
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
