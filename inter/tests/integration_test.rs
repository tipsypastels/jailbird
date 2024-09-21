use jailbird_choice::*;
use jailbird_inter::*;

#[test]
fn always_cooperate() {
    let mut inter = Interpreter::new();
    let always_cooperate = inter.bind("return COOPERATE;");
    let choice = inter.call(&always_cooperate, DummyCtx).unwrap();

    assert_eq!(choice, Cooperate);
}

#[test]
fn tit_for_tat() {
    let mut inter = Interpreter::new();
    let tit_for_tat = inter.bind("return context.otherPlayer.choices.at(-1) ?? COOPERATE;");

    let first_ctx = ChoicesContext::new(&[], &[]);
    let first_choice = inter.call(&tit_for_tat, first_ctx).unwrap();

    assert_eq!(first_choice, Cooperate);

    let later_ctx = ChoicesContext::new(&[], &[Defect]);
    let later_choice = inter.call(&tit_for_tat, later_ctx).unwrap();

    assert_eq!(later_choice, Defect);
}

#[derive(Copy, Clone)]
struct DummyCtx;

impl Context for DummyCtx {
    type Turn = Self;
    type Player = Self;

    fn turn(&self) -> Self::Turn {
        *self
    }

    fn this_player(&self) -> &Self::Player {
        self
    }

    fn other_player(&self) -> &Self::Player {
        self
    }
}

impl TurnContext for DummyCtx {
    fn current(&self) -> i32 {
        1
    }

    fn total(&self) -> i32 {
        2
    }
}

impl PlayerContext for DummyCtx {
    fn choices(&self) -> &[Choice] {
        &[]
    }
}

struct ChoicesContext(ChoicesPlayerContext, ChoicesPlayerContext);

impl ChoicesContext {
    fn new(this: &'static [Choice], other: &'static [Choice]) -> Self {
        Self(ChoicesPlayerContext(this), ChoicesPlayerContext(other))
    }
}

impl Context for ChoicesContext {
    type Turn = DummyCtx;
    type Player = ChoicesPlayerContext;

    fn turn(&self) -> Self::Turn {
        DummyCtx
    }

    fn this_player(&self) -> &Self::Player {
        &self.0
    }

    fn other_player(&self) -> &Self::Player {
        &self.1
    }
}

struct ChoicesPlayerContext(&'static [Choice]);

impl PlayerContext for ChoicesPlayerContext {
    fn choices(&self) -> &[Choice] {
        self.0
    }
}
