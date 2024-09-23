use jailbird_choice::Choice;

pub trait Context {
    type Turn: Turn;
    type Player: Player;

    fn turn(&self) -> Self::Turn;
    fn this_player(&self) -> Self::Player;
    fn other_player(&self) -> Self::Player;
}

pub trait Turn {
    fn cur(&self) -> i32;
    fn max(&self) -> i32;
}

pub trait Player {
    fn choices(&self) -> &[Choice];
}
