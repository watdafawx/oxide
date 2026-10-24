use mastery_one_lookup::probe::Counter;
use mastery_one_lookup::word_count;

#[test]
fn word_count_uses_one_operation_per_word() {
    let mut counter = Counter::new();
    word_count("a b a c a b", &mut counter);
    let ops = counter.ops();
    assert_eq!(counter.get("a"), Some(3));
    assert_eq!(counter.get("b"), Some(2));
    assert_eq!(counter.get("c"), Some(1));
    assert!(ops <= 6, "{ops} Counter operations for 6 words, expected at most 6");
}
