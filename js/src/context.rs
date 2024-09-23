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
    fn score(&self) -> i32;
    fn choices(&self) -> &[Choice];

    fn ever_cooperated(&self) -> bool {
        self.choices().iter().any(|c| c.is_cooperate())
    }

    fn ever_defected(&self) -> bool {
        self.choices().iter().any(|c| c.is_defect())
    }
}
