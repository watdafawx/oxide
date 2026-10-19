

/// A new player's kit: a wooden sword, then bread, then a torch.
pub fn starter_kit() -> Vec<&'static str> {
    // TODO: make `let mut kit = vec!["wooden_sword"];`, push "bread" and "torch", return kit
    todo!()
}

/// What is in hotbar slot 3 (index 2)? None if the hotbar has fewer than three items.
pub fn third_slot<'a>(slots: &[&'a str]) -> Option<&'a str> {
    // TODO: slots.get(2) gives Option<&&str>; .copied() makes it Option<&str>
    todo!()
}

/// Adds up every stack count in the inventory. An empty inventory has 0 items.
pub fn total_items(stacks: &[u32]) -> u32 {
    // TODO: make `let mut total = 0;`, add each count in a `for` loop, return total
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn podium_pushes_two_more() {
        assert_eq!(starter_kit(), vec!["wooden_sword", "bread", "torch"]);
    }

    #[test]
    fn third_uses_get() {
        assert_eq!(third_slot(&["sword", "bow", "torch", "bread"]), Some("torch"));
        assert_eq!(third_slot(&["sword", "bow"]), None, "two items means no third slot");
        assert_eq!(third_slot(&[]), None);
    }

    #[test]
    fn sum_all_loops_over_the_vec() {
        assert_eq!(total_items(&[64, 16, 3, 1]), 84);
        assert_eq!(total_items(&[1, 1, 62]), 64);
        assert_eq!(total_items(&[]), 0);
    }
}
