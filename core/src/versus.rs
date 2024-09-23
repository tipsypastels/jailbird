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
            return VersusState::Done;
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
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub enum VersusState {
    Ongoing(Versus),
    // TODO
    Done,
}
