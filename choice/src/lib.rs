use implicit_clone::ImplicitClone;

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Choice {
    Cooperate,
    Defect,
}

pub use Choice::{Cooperate, Defect};

impl Choice {
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
