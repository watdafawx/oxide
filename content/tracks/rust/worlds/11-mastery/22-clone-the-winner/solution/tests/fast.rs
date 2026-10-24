use mastery_clone_winner::longest;
use mastery_clone_winner::probe::{Name, clones};

#[test]
fn longest_clones_at_most_once() {
    let names: Vec<Name> = ["ann", "bob1", "carol", "dorian", "eleanora", "frank"]
        .iter()
        .map(|s| Name(s.to_string()))
        .collect();
    let before = clones();
    let best = longest(&names);
    let used = clones() - before;
    assert_eq!(best, Some(Name("eleanora".to_string())));
    assert!(used <= 1, "{used} clones, expected at most 1");
    assert_eq!(longest(&[]), None);
}
