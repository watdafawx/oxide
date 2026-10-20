

/// A new player's kit: a wooden sword, then bread, then a torch.
pub fn starter_kit() -> Vec<&'static str> {
    // TODO: start the kit with the wooden sword, push bread and torch, return it
    todo!()
}

/// What is in hotbar slot 3 (index 2)? None if the hotbar has fewer than three items.
pub fn third_slot<'a>(slots: &[&'a str]) -> Option<&'a str> {
    // TODO: read slot index 2 without panicking; return the item itself, not a reference to it
    todo!()
}

/// Adds up every stack count in the inventory. An empty inventory has 0 items.
pub fn total_items(stacks: &[u32]) -> u32 {
    // TODO: add every stack count in a `for` loop and return the total
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_kit_adds_bread_and_torch() {
        assert_eq!(starter_kit(), vec!["wooden_sword", "bread", "torch"]);
    }

    #[test]
    fn third_slot_is_none_on_short_hotbar() {
        assert_eq!(third_slot(&["sword", "bow", "torch", "bread"]), Some("torch"));
        assert_eq!(third_slot(&["sword", "bow"]), None, "two items means no third slot");
        assert_eq!(third_slot(&[]), None);
    }

    #[test]
    fn total_items_adds_every_stack() {
        assert_eq!(total_items(&[64, 16, 3, 1]), 84);
        assert_eq!(total_items(&[1, 1, 62]), 64);
        assert_eq!(total_items(&[]), 0);
    }
}
