/// A stack starts at 10 items, then you pick up 5 more.
pub fn stack_after_pickup() -> i32 {
    let mut stack = 10;
    stack += 5;
    stack
}

/// Length of an item name without leading/trailing spaces.
pub fn item_name_len(name: &str) -> usize {
    let name = name.trim();
    name.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_after_pickup_adds_five() {
        assert_eq!(stack_after_pickup(), 15, "10 items + 5 picked up");
    }

    #[test]
    fn item_name_len_ignores_padding() {
        assert_eq!(item_name_len("  oak_log  "), 7);
        assert_eq!(item_name_len("cobblestone"), 11);
    }
}
