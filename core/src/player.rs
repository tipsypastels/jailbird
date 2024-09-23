use crate::Strategy;
use implicit_clone::{unsync::IArray, ImplicitClone};
use jailbird_choice::Choice;
use std::iter::once;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct Player {
    pub score: u32,
    pub strategy: Strategy,
    pub choices: IArray<Choice>,
    pub ever_cooperated: bool,
    pub ever_defected: bool,
}

impl Player {
    pub fn new(strategy: Strategy) -> Self {
        Self {
            score: 0,
            strategy,
            choices: IArray::default(),
            ever_cooperated: false,
            ever_defected: false,
        }
    }

    pub fn next(self, gain: u32, choice: Choice) -> Self {
        let Self {
            score,
            strategy,
            choices,
            ever_cooperated,
            ever_defected,
        } = self;

        Self {
            score: score + gain,
            strategy,
            choices: choices.iter().chain(once(choice)).collect(),
            ever_cooperated: ever_cooperated || choice.is_cooperate(),
            ever_defected: ever_defected || choice.is_defect(),
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
        &self.choices
    }

    fn ever_cooperated(&self) -> bool {
        self.ever_cooperated
    }

    fn ever_defected(&self) -> bool {
        self.ever_defected
    }
}
