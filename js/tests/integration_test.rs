use jailbird_choice::*;
use jailbird_js::*;

#[test]
fn always_cooperate() {
    let mut js = Js::new();
    let always_cooperate = js.bind("return COOPERATE;");
    let choice = js.call(&always_cooperate, DummyCtx).unwrap();

    assert_eq!(choice, Cooperate);
}

#[test]
fn tit_for_tat() {
    let mut js = Js::new();
    let tit_for_tat = js.bind("return context.otherPlayer.choices.at(-1) ?? COOPERATE;");

    let first_ctx = ChoicesContext::new(&[], &[]);
    let first_choice = js.call(&tit_for_tat, first_ctx).unwrap();

    assert_eq!(first_choice, Cooperate);

    let later_ctx = ChoicesContext::new(&[], &[Defect]);
    let later_choice = js.call(&tit_for_tat, later_ctx).unwrap();

    assert_eq!(later_choice, Defect);
}

#[test]
fn grudger() {
    let mut js = Js::new();
    let grudger = js.bind("return context.otherPlayer.everDefected ? DEFECT : COOPERATE;");

    let first_ctx = ChoicesContext::new(&[], &[]);
    let first_choice = js.call(&grudger, first_ctx).unwrap();

    assert_eq!(first_choice, Cooperate);

    let later_ctx = ChoicesContext::new(&[], &[Cooperate]);
    let later_choice = js.call(&grudger, later_ctx).unwrap();

    assert_eq!(later_choice, Cooperate);

    let even_later_ctx = ChoicesContext::new(&[], &[Defect, Cooperate]);
    let even_later_choice = js.call(&grudger, even_later_ctx).unwrap();

    assert_eq!(even_later_choice, Defect);
}

#[derive(Copy, Clone)]
struct DummyCtx;

impl Context for DummyCtx {
    type Turn = Self;
    type Player = Self;

    fn turn(&self) -> Self::Turn {
        *self
    }

    fn this_player(&self) -> Self::Player {
        *self
    }

    fn other_player(&self) -> Self::Player {
        *self
    }
}

impl Turn for DummyCtx {
    fn cur(&self) -> i32 {
        1
    }

    fn max(&self) -> i32 {
        2
    }
}

impl Player for DummyCtx {
    fn score(&self) -> i32 {
        0
    }

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

    fn this_player(&self) -> Self::Player {
        self.0
    }

    fn other_player(&self) -> Self::Player {
        self.1
    }
}

#[derive(Copy, Clone)]
struct ChoicesPlayerContext(&'static [Choice]);

impl Player for ChoicesPlayerContext {
    fn score(&self) -> i32 {
        0
    }

    fn choices(&self) -> &[Choice] {
        self.0
    }
}
