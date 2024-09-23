use crate::{Player, Turn};

#[derive(Debug)]
#[non_exhaustive]
pub struct View {
    pub turn: Turn,
    pub this_player: Player,
    pub other_player: Player,
}

#[cfg(feature = "js")]
impl jailbird_js::View for View {
    type Turn = Turn;
    type Player = Player;

    fn turn(&self) -> Turn {
        self.turn.clone()
    }

    fn this_player(&self) -> Player {
        self.this_player.clone()
    }

    fn other_player(&self) -> Player {
        self.other_player.clone()
    }
}
