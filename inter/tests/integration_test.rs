use jailbird_inter::*;

#[test]
fn always_cooperate() {
    let mut inter = Interpreter::<DummyCtx>::new();
    let always_cooperate = inter.bind("return COOPERATE;");
    let choice = inter.call(&always_cooperate, DummyCtx).unwrap();

    assert_eq!(choice, Choice::Cooperate);
}

#[test]
fn tit_for_tat() {
    let mut inter = Interpreter::<ChoicesContext>::new();
    let tit_for_tat = inter.bind("return context.otherPlayer.choices.at(-1) ?? COOPERATE;");

    let first_ctx = ChoicesContext::new(&[], &[]);
    let first_choice = inter.call(&tit_for_tat, first_ctx).unwrap();

    assert_eq!(first_choice, Choice::Cooperate);

    let later_ctx = ChoicesContext::new(&[], &[Choice::Defect]);
    let later_choice = inter.call(&tit_for_tat, later_ctx).unwrap();

    assert_eq!(later_choice, Choice::Defect);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Choice {
    Cooperate,
    Defect,
}

impl ChoiceContext for Choice {
    fn cooperate() -> Self {
        Self::Cooperate
    }

    fn defect() -> Self {
        Self::Defect
    }

    fn is_cooperate(self) -> bool {
        matches!(self, Self::Cooperate)
    }
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
    type Choice = Choice;

    fn choices(&self) -> &[Self::Choice] {
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
    type Choice = Choice;

    fn choices(&self) -> &[Self::Choice] {
        self.0
    }
}
