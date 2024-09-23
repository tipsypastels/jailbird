use jailbird_core::*;

#[test]
fn simple() {
    let mut rt = Runtime::new();
    let turns = 5;

    let always_cooperate = rt.always_cooperate();
    let always_defect = rt.always_defect();

    let versus = Versus::new(always_cooperate.clone(), always_defect.clone(), turns);
    let ending = versus.next_to_ending(&mut rt);
    let (player1, player2) = ending.into_win_lose().unwrap();

    assert!(!player1.won);
    assert_eq!(player1.score, 0);

    assert!(player2.won);
    assert_eq!(player2.score, 25);
}
