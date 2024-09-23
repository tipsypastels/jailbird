use crate::Strategy;
use implicit_clone::{unsync::IArray, ImplicitClone};
use jailbird_choice::Choice;
use std::{
    iter::{once, Copied},
    ops::Deref,
    slice,
};

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct Player {
    pub score: u32,
    pub strategy: Strategy,
    pub choices: Choices,
    pub ever_cooperated: bool,
    pub ever_defected: bool,
}

impl Player {
    pub fn new(strategy: Strategy) -> Self {
        Self {
            score: 0,
            strategy,
            choices: Choices(Default::default()),
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
            choices: choices.next(choice),
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
impl jailbird_js::PlayerView for Player {
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

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct Choices(IArray<Choice>);

impl Choices {
    fn next(self, choice: Choice) -> Self {
        Self(self.0.iter().chain(once(choice)).collect())
    }

    pub fn as_slice(&self) -> &[Choice] {
        &self.0
    }

    pub fn iter(&self) -> Copied<slice::Iter<'_, Choice>> {
        self.as_slice().iter().copied()
    }
}

impl FromIterator<Choice> for Choices {
    fn from_iter<T: IntoIterator<Item = Choice>>(iter: T) -> Self {
        Self(IArray::<Choice>::from_iter(iter))
    }
}

impl Deref for Choices {
    type Target = [Choice];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
