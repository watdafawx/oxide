/// A stack starts at 10 items, then you pick up 5 more.
pub fn stack_after_pickup() -> i32 {
    // TODO: make `stack` mutable with `mut`, then add 5 to it
    let stack = 10;
    stack
}

/// Length of an item name without leading/trailing spaces.
pub fn item_name_len(name: &str) -> usize {
    // TODO: shadow `name` with `name.trim()`, then return its length
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_is_added_to_score() {
        assert_eq!(stack_after_pickup(), 15, "10 items + 5 picked up");
    }

    #[test]
    fn trimmed_len_ignores_spaces() {
        assert_eq!(item_name_len("  oak_log  "), 7);
        assert_eq!(item_name_len("cobblestone"), 11);
    }
}
