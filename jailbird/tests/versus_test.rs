use jailbird::*;

#[test]
fn simple() {
    let mut rt = Runtime::new();
    let turns = 5;

    let always_cooperate = rt.always_cooperate();
    let always_defect = rt.always_defect();

    let versus = Versus::new(always_cooperate.clone(), always_defect.clone(), turns);
    let ending = versus.next_to_ending(&mut rt);

    assert!(!ending.player1.won);
    assert_eq!(ending.player1.score, 0);

    assert!(ending.player2.won);
    assert_eq!(ending.player2.score, 25);
}
