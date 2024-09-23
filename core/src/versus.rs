use std::ops::Deref;

use crate::{Player, Runtime, Turn, View};
use implicit_clone::ImplicitClone;
use jailbird_choice::{Choice, ChoiceMatrix};

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct Versus {
    pub player1: Player,
    pub player2: Player,
    pub turn: Turn,
    matrix: ChoiceMatrix,
}

impl Versus {
    pub fn new(
        player1: impl Into<Player>,
        player2: impl Into<Player>,
        turn: impl Into<Turn>,
    ) -> Self {
        Self {
            player1: player1.into(),
            player2: player2.into(),
            turn: turn.into(),
            matrix: Choice::matrix,
        }
    }

    pub fn choice_matrix(self, matrix: ChoiceMatrix) -> Self {
        Self {
            player1: self.player1,
            player2: self.player2,
            turn: self.turn,
            matrix,
        }
    }

    pub fn next(self, rt: &mut Runtime) -> VersusState {
        let Some(turn) = self.turn.next() else {
            let (player1, player2) = (self.player1, self.player2);

            macro_rules! win_lose {
                ($p1_won:literal) => {
                    VersusEnding::WinLose {
                        player1: VersusWinnerOrLoser {
                            player: player1,
                            won: $p1_won,
                        },
                        player2: VersusWinnerOrLoser {
                            player: player2,
                            won: !$p1_won,
                        },
                    }
                };
            }

            return match () {
                () if player1.score > player2.score => VersusState::Ending(win_lose!(true)),
                () if player2.score > player1.score => VersusState::Ending(win_lose!(false)),
                () => VersusState::Ending(VersusEnding::Tie { player1, player2 }),
            };
        };

        macro_rules! view {
            ($this:ident $other:ident) => {
                View {
                    turn: turn.clone(),
                    this_player: self.$this.clone(),
                    other_player: self.$other.clone(),
                }
            };
        }

        let c1 = self.player1.strategy.call(rt, view!(player1 player2));
        let c2 = self.player2.strategy.call(rt, view!(player2 player1));
        let (g1, g2) = (self.matrix)(c1, c2);

        let player1 = self.player1.next(g1, c1);
        let player2 = self.player2.next(g2, c2);

        VersusState::Ongoing(Self {
            player1,
            player2,
            turn,
            matrix: self.matrix,
        })
    }

    pub fn next_to_ending(self, rt: &mut Runtime) -> VersusEnding {
        match self.next(rt) {
            VersusState::Ongoing(versus) => versus.next_to_ending(rt),
            VersusState::Ending(ending) => ending,
        }
    }
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub enum VersusState {
    Ongoing(Versus),
    Ending(VersusEnding),
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub enum VersusEnding {
    WinLose {
        player1: VersusWinnerOrLoser,
        player2: VersusWinnerOrLoser,
    },
    Tie {
        player1: Player,
        player2: Player,
    },
}

impl VersusEnding {
    pub fn into_win_lose(self) -> Result<(VersusWinnerOrLoser, VersusWinnerOrLoser), Self> {
        match self {
            Self::WinLose { player1, player2 } => Ok((player1, player2)),
            other => Err(other),
        }
    }

    pub fn into_tie(self) -> Result<(Player, Player), Self> {
        match self {
            Self::Tie { player1, player2 } => Ok((player1, player2)),
            other => Err(other),
        }
    }
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct VersusWinnerOrLoser {
    pub player: Player,
    pub won: bool,
}

impl Deref for VersusWinnerOrLoser {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.player
    }
}
