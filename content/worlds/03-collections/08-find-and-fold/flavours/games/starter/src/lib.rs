/// The first stack size bigger than `limit`.
pub fn first_stack_over(stacks: &[u32], limit: u32) -> Option<u32> {
    // TODO: stacks.iter().copied().find(|n| ...)
    todo!()
}

/// Which slot holds `name`? None if it is not in the inventory.
pub fn slot_of(items: &[String], name: &str) -> Option<usize> {
    // TODO: items.iter().position(|n| ...): compare n.as_str() == name
    todo!()
}

/// Is any tool one hit from breaking (durability 1 or less)?
pub fn has_fragile_tool(durability: &[u32]) -> bool {
    // TODO: durability.iter().any(|d| ...)
    todo!()
}

/// Is every stack at least `full` items? An empty list counts as yes.
pub fn all_full_stacks(counts: &[u32], full: u32) -> bool {
    // TODO: counts.iter().all(|c| ...)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn first_stack_over_and_slot_of_search() {
        assert_eq!(first_stack_over(&[4, 8, 15, 16], 10), Some(15));
        assert_eq!(first_stack_over(&[4, 8], 10), None);
        let items = strings(&["sword", "bow", "torch"]);
        assert_eq!(slot_of(&items, "bow"), Some(1));
        assert_eq!(slot_of(&items, "elytra"), None);
    }

    #[test]
    fn fragile_tool_and_full_stacks_checks() {
        assert!(has_fragile_tool(&[30, 1, 12]));
        assert!(!has_fragile_tool(&[30, 2, 12]));
        assert!(all_full_stacks(&[64, 64, 70], 64));
        assert!(!all_full_stacks(&[64, 63], 64));
        assert!(all_full_stacks(&[], 64), "no stacks, nothing missing");
    }
}
