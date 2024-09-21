use jailbird_choice::Choice;

pub trait Context {
    type Turn: TurnContext;
    type Player: PlayerContext;

    fn turn(&self) -> Self::Turn;
    fn this_player(&self) -> &Self::Player;
    fn other_player(&self) -> &Self::Player;
}

pub trait TurnContext {
    fn current(&self) -> i32;
    fn total(&self) -> i32;
}

pub trait PlayerContext {
    fn choices(&self) -> &[Choice];
}
