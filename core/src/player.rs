use crate::Strategy;
use implicit_clone::{unsync::IArray, ImplicitClone};
use jailbird_choice::Choice;
use std::iter::once;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct Player {
    score: u32,
    strategy: Strategy,
    history: History,
}

impl Player {
    pub fn new(strategy: Strategy) -> Self {
        Self {
            score: 0,
            strategy,
            history: Default::default(),
        }
    }

    pub fn next(self, gain: u32, choice: Choice) -> Self {
        let Self {
            score,
            strategy,
            history,
        } = self;

        Self {
            score: score + gain,
            strategy,
            history: History {
                choices: history.choices.iter().chain(once(choice)).collect(),
                ever_cooperated: history.ever_cooperated || choice.is_cooperate(),
                ever_defected: history.ever_defected || choice.is_defect(),
            },
        }
    }
}

impl From<Strategy> for Player {
    fn from(strategy: Strategy) -> Self {
        Self::new(strategy)
    }
}

#[cfg(feature = "inter")]
impl jailbird_inter::PlayerContext for Player {
    fn choices(&self) -> &[Choice] {
        &self.history.choices
    }
}

#[derive(Debug, Default, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct History {
    pub choices: IArray<Choice>,
    pub ever_cooperated: bool,
    pub ever_defected: bool,
}
