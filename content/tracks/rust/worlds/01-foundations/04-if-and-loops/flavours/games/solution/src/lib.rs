/// 64 -> "Full", 0 -> "Empty", anything else -> the count as text ("7")
pub fn slot_label(count: u32) -> String {
    if count == 64 {
        "Full".to_string()
    } else if count == 0 {
        "Empty".to_string()
    } else {
        count.to_string()
    }
}

/// A staircase of `steps` steps needs 1 + 2 + ... + steps blocks.
/// staircase_blocks(4) -> 10
pub fn staircase_blocks(steps: u32) -> u32 {
    let mut total = 0;
    for i in 1..=steps {
        total += i;
    }
    total
}

/// How many swings with `damage` per hit until a block with `block_hp` breaks?
/// swings_to_break(10, 3) -> 4 (10, 7, 4, 1, then gone)
pub fn swings_to_break(mut block_hp: i32, damage: i32) -> u32 {
    let mut swings = 0;
    while block_hp > 0 {
        block_hp -= damage;
        swings += 1;
    }
    swings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_label_names_full_empty_or_count() {
        assert_eq!(slot_label(64), "Full");
        assert_eq!(slot_label(0), "Empty");
        assert_eq!(slot_label(7), "7");
        assert_eq!(slot_label(32), "32");
    }

    #[test]
    fn staircase_needs_one_through_n_blocks() {
        assert_eq!(staircase_blocks(4), 10);
        assert_eq!(staircase_blocks(0), 0, "an empty range adds nothing");
    }

    #[test]
    fn swings_to_break_counts_until_block_gone() {
        assert_eq!(swings_to_break(10, 3), 4);
        assert_eq!(swings_to_break(0, 3), 0, "already broken, no swings");
    }
}
