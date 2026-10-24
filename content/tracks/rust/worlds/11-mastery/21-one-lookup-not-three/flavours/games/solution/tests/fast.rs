use mastery_one_lookup::count_blocks;
use mastery_one_lookup::probe::Counter;

#[test]
fn count_blocks_uses_one_operation_per_block() {
    let mut counter = Counter::new();
    count_blocks("stone dirt stone sand stone dirt", &mut counter);
    let ops = counter.ops();
    assert_eq!(counter.get("stone"), Some(3));
    assert_eq!(counter.get("dirt"), Some(2));
    assert_eq!(counter.get("sand"), Some(1));
    assert!(ops <= 6, "{ops} Counter operations for 6 blocks, expected at most 6");
}
