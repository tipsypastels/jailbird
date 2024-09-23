#![warn(missing_debug_implementations)]

mod function;
mod strategy;

use jailbird_choice::Choice::{Cooperate, Defect};
pub use strategy::{Strategy, StrategyBuilder};

#[derive(Debug)]
pub struct Runtime {
    #[cfg(feature = "inter")]
    inter: jailbird_inter::Interpreter,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "inter")]
            inter: jailbird_inter::Interpreter::new(),
        }
    }

    pub fn strategy<F>(&mut self, f: F) -> Strategy
    where
        F: FnOnce(StrategyBuilder<strategy::Step0>) -> Strategy,
    {
        f(StrategyBuilder::new(self))
    }

    pub fn always_cooperate(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Always Cooperate")
                .desc("Always cooperates.")
                .native_with_js_example(|_| Cooperate, "return COOPERATE;")
                .build()
        })
    }

    pub fn always_defect(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Always Defect")
                .desc("Always defects.")
                .native_with_js_example(|_| Defect, "return DEFECT;")
                .build()
        })
    }

    pub fn tit_for_tat(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Tit For Tat")
                .desc(
                    "Copies the other player's last move, or cooperates if this is the first turn.",
                )
                .native_with_js_example(
                    |c| c.other_player.last().copied().unwrap_or(Cooperate),
                    "return context.otherPlayer.choices.at(-1) ?? COOPERATE",
                )
                .build()
        })
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
