use jailbird_inter::*;

#[test]
fn always_cooperate() {
    let mut inter = Interpreter::<DummyCtx>::new();
    let always_cooperate = inter.bind("return COOPERATE;");
    let choice = inter.call(&always_cooperate, DummyCtx).unwrap();

    assert_eq!(choice, Choice::Cooperate);
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

    fn this_player(&self) -> Self::Player {
        *self
    }

    fn other_player(&self) -> Self::Player {
        *self
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
