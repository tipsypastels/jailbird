use crate::Strategy;
use implicit_clone::{unsync::IArray, ImplicitClone};
use jailbird_choice::Choice;
use std::{iter::once, ops::Deref};

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct Player {
    pub score: u32,
    pub strategy: Strategy,
    pub history: History,
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
                choices: history.iter().chain(once(choice)).collect(),
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

#[cfg(feature = "js")]
impl jailbird_js::Player for Player {
    fn score(&self) -> i32 {
        self.score as i32
    }

    fn choices(&self) -> &[Choice] {
        &self.history
    }

    fn ever_cooperated(&self) -> bool {
        self.history.ever_cooperated
    }

    fn ever_defected(&self) -> bool {
        self.history.ever_defected
    }
}

#[derive(Debug, Default, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct History {
    pub choices: IArray<Choice>,
    pub ever_cooperated: bool,
    pub ever_defected: bool,
}

impl Deref for History {
    type Target = IArray<Choice>;

    fn deref(&self) -> &Self::Target {
        &self.choices
    }
}
