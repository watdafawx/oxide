/// 64 -> "Full", 0 -> "Empty", anything else -> the count as text ("7")
pub fn slot_label(count: u32) -> String {
    // TODO: if / else if / else chain
    todo!()
}

/// A staircase of `steps` steps needs 1 + 2 + ... + steps blocks.
/// staircase_blocks(4) -> 10
pub fn staircase_blocks(steps: u32) -> u32 {
    // TODO: for i in 1..=steps { ... }
    todo!()
}

/// How many swings with `damage` per hit until a block with `block_hp` breaks?
/// swings_to_break(10, 3) -> 4 (10, 7, 4, 1, then gone)
pub fn swings_to_break(mut block_hp: i32, damage: i32) -> u32 {
    // TODO: count swings in a `while block_hp > 0` loop
    todo!()
}

/// Smallest number of 64-item stacks (starting at 1) that holds `items`.
pub fn stacks_to_hold(items: u32) -> u32 {
    // TODO: `loop` with `break n;`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_words() {
        assert_eq!(slot_label(64), "Full");
        assert_eq!(slot_label(0), "Empty");
        assert_eq!(slot_label(7), "7");
        assert_eq!(slot_label(32), "32");
    }

    #[test]
    fn sum_to_adds_range() {
        assert_eq!(staircase_blocks(4), 10);
        assert_eq!(staircase_blocks(0), 0, "an empty range adds nothing");
    }

    #[test]
    fn collatz_counts_steps() {
        assert_eq!(swings_to_break(10, 3), 4);
        assert_eq!(swings_to_break(0, 3), 0, "already broken, no swings");
    }

    #[test]
    fn first_square_over_breaks_with_value() {
        assert_eq!(stacks_to_hold(65), 2, "64 fits one stack, 65 needs a second");
        assert_eq!(stacks_to_hold(200), 4);
        assert_eq!(stacks_to_hold(64), 1);
    }
}
