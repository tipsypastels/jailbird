use implicit_clone::ImplicitClone;

pub type ChoiceMatrix = fn(Choice, Choice) -> (u32, u32);

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Choice {
    Cooperate,
    Defect,
}

pub use Choice::{Cooperate, Defect};

impl Choice {
    pub fn matrix(a: Self, b: Self) -> (u32, u32) {
        match (a, b) {
            (Cooperate, Cooperate) => (3, 3),
            (Cooperate, Defect) => (0, 5),
            (Defect, Cooperate) => (5, 0),
            (Defect, Defect) => (1, 1),
        }
    }

    pub fn is_cooperate(self) -> bool {
        matches!(self, Cooperate)
    }

    pub fn is_defect(self) -> bool {
        matches!(self, Defect)
    }

    pub fn from_bool(b: bool) -> Self {
        if b {
            Cooperate
        } else {
            Defect
        }
    }
}
