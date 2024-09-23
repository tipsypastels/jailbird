use jailbird_choice::*;
use jailbird_js::*;

#[test]
fn always_cooperate() {
    let mut js = Js::new();
    let always_cooperate = js.bind("return COOPERATE;");
    let choice = js.call(&always_cooperate, DummyView).unwrap();

    assert_eq!(choice, Cooperate);
}

#[test]
fn tit_for_tat() {
    let mut js = Js::new();
    let tit_for_tat = js.bind("return view.otherPlayer.choices.at(-1) ?? COOPERATE;");

    let first_view = ChoicesView::new(&[], &[]);
    let first_choice = js.call(&tit_for_tat, first_view).unwrap();

    assert_eq!(first_choice, Cooperate);

    let later_view = ChoicesView::new(&[], &[Defect]);
    let later_choice = js.call(&tit_for_tat, later_view).unwrap();

    assert_eq!(later_choice, Defect);
}

#[test]
fn grudger() {
    let mut js = Js::new();
    let grudger = js.bind("return view.otherPlayer.everDefected ? DEFECT : COOPERATE;");

    let first_view = ChoicesView::new(&[], &[]);
    let first_choice = js.call(&grudger, first_view).unwrap();

    assert_eq!(first_choice, Cooperate);

    let later_view = ChoicesView::new(&[], &[Cooperate]);
    let later_choice = js.call(&grudger, later_view).unwrap();

    assert_eq!(later_choice, Cooperate);

    let even_later_view = ChoicesView::new(&[], &[Defect, Cooperate]);
    let even_later_choice = js.call(&grudger, even_later_view).unwrap();

    assert_eq!(even_later_choice, Defect);
}

#[derive(Copy, Clone)]
struct DummyView;

impl View for DummyView {
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

impl TurnView for DummyView {
    fn cur(&self) -> i32 {
        1
    }

    fn max(&self) -> i32 {
        2
    }
}

impl PlayerView for DummyView {
    fn score(&self) -> i32 {
        0
    }

    fn choices(&self) -> &[Choice] {
        &[]
    }
}

struct ChoicesView(ChoicesPlayerView, ChoicesPlayerView);

impl ChoicesView {
    fn new(this: &'static [Choice], other: &'static [Choice]) -> Self {
        Self(ChoicesPlayerView(this), ChoicesPlayerView(other))
    }
}

impl View for ChoicesView {
    type Turn = DummyView;
    type Player = ChoicesPlayerView;

    fn turn(&self) -> Self::Turn {
        DummyView
    }

    fn this_player(&self) -> Self::Player {
        self.0
    }

    fn other_player(&self) -> Self::Player {
        self.1
    }
}

#[derive(Copy, Clone)]
struct ChoicesPlayerView(&'static [Choice]);

impl PlayerView for ChoicesPlayerView {
    fn score(&self) -> i32 {
        0
    }

    fn choices(&self) -> &[Choice] {
        self.0
    }
}
