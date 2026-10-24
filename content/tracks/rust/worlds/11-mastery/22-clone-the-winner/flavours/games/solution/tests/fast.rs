use mastery_clone_winner::longest_name;
use mastery_clone_winner::probe::{Name, clones};

#[test]
fn longest_name_clones_at_most_once() {
    let names: Vec<Name> = ["oak", "birch", "spruce", "jungle", "dark_oak", "acacia"]
        .iter()
        .map(|s| Name(s.to_string()))
        .collect();
    let before = clones();
    let best = longest_name(&names);
    let used = clones() - before;
    assert_eq!(best, Some(Name("dark_oak".to_string())));
    assert!(used <= 1, "{used} clones, expected at most 1");
    assert_eq!(longest_name(&[]), None);
}
