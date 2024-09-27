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
                    "return view.otherPlayer.choices.at(-1) ?? COOPERATE",
                )
                .build()
        })
    }

    pub fn suspicious_tit_for_tat(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Suspicious Tit For Tat")
                .desc("Copies the other player's last move, or defects if this is the first turn.")
                .builtin(
                    impls::suspicious_tit_for_tat,
                    "return view.otherPlayer.choices.at(-1) ?? DEFECT",
                )
                .build()
        })
    }

    pub fn tit_for_two_tats(&mut self) -> Strategy {
        self.strategy(|s| {
            s.name("Tit For Two Tats")
                .desc(
                    "Defects if the other player's last two moves have been defections, or cooperates.",
                )
                .builtin(
                    impls::tit_for_two_tats,
                    "return view.otherPlayer.choices.at(-1) === DEFECT && view.otherPlayer.choices.at(-2) === DEFECT ? DEFECT : COOPERATE",
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
                    "return view.otherPlayer.everDefected ? DEFECT : COOPERATE;",
                )
                .build()
        })
    }
}

// Don't accidentally use jailbird_js view traits in here.
mod impls {
    use crate::View;
    use jailbird_choice::*;

    pub fn always_cooperate(_: View) -> Choice {
        Cooperate
    }

    pub fn always_defect(_: View) -> Choice {
        Defect
    }

    pub fn tit_for_tat(view: View) -> Choice {
        view.other_player
            .choices
            .last()
            .copied()
            .unwrap_or(Cooperate)
    }

    pub fn suspicious_tit_for_tat(view: View) -> Choice {
        view.other_player.choices.last().copied().unwrap_or(Defect)
    }

    pub fn tit_for_two_tats(view: View) -> Choice {
        let mut iter = view.other_player.choices.iter().rev();

        if let (Some(Defect), Some(Defect)) = (iter.next(), iter.next()) {
            Defect
        } else {
            Cooperate
        }
    }

    pub fn grudger(view: View) -> Choice {
        if view.other_player.ever_defected {
            Defect
        } else {
            Cooperate
        }
    }
}
