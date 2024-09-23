use crate::{Runtime, Strategy};

impl Runtime {
    pub fn always_cooperate(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Always Cooperate")
                .desc("Always cooperates.")
                .builtin(impls::always_cooperate, "return COOPERATE;")
                .build()
        })
    }

    pub fn always_defect(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Always Defect")
                .desc("Always defects.")
                .builtin(impls::always_defect, "return DEFECT;")
                .build()
        })
    }

    pub fn tit_for_tat(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Tit For Tat")
                .desc(
                    "Copies the other player's last move, or cooperates if this is the first turn.",
                )
                .builtin(
                    impls::tit_for_tat,
                    "return context.otherPlayer.choices.at(-1) ?? COOPERATE",
                )
                .build()
        })
    }

    pub fn grudger(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Grudger")
                .desc("Defects if the other player has ever defected, otherwise cooperates.")
                .builtin(
                    impls::grudger,
                    "return context.otherPlayer.everDefected ? DEFECT : COOPERATE;",
                )
                .build()
        })
    }
}

// Don't accidentally use jailbird_js context traits in here.
mod impls {
    use crate::function::Context;
    use jailbird_choice::*;

    pub fn always_cooperate(_: Context) -> Choice {
        Cooperate
    }

    pub fn always_defect(_: Context) -> Choice {
        Defect
    }

    pub fn tit_for_tat(ctx: Context) -> Choice {
        ctx.other_player
            .history
            .last()
            .copied()
            .unwrap_or(Cooperate)
    }

    pub fn grudger(ctx: Context) -> Choice {
        if ctx.other_player.history.ever_defected {
            Defect
        } else {
            Cooperate
        }
    }
}
