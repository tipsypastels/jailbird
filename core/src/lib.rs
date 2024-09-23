#![warn(missing_debug_implementations)]

mod function;
mod player;
mod strategy;
mod turn;
mod versus;

pub use jailbird_choice::*;
pub use player::{History, Player};
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

    pub fn always_cooperate(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Always Cooperate")
                .desc("Always cooperates.")
                .builtin(|_| Cooperate, "return COOPERATE;")
                .build()
        })
    }

    pub fn always_defect(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Always Defect")
                .desc("Always defects.")
                .builtin(|_| Defect, "return DEFECT;")
                .build()
        })
    }

    pub fn tit_for_tat(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Tit For Tat")
                .desc(
                    "Copies the other player's last move, or cooperates if this is the first turn.",
                )
                .builtin(
                    |c| c.other_player.history.last().copied().unwrap_or(Cooperate),
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
