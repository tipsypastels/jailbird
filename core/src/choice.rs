use implicit_clone::ImplicitClone;

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub enum Choice {
    Cooperate,
    Defect,
}
