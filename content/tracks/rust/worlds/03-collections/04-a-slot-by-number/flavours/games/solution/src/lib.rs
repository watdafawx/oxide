/// The stack size in hotbar slot `slot` (0 is the first). None if the hotbar is shorter.
pub fn stack_in_slot(hotbar: &[u32], slot: usize) -> Option<u32> {
    hotbar.get(slot).copied()
}

/// The same, but a slot past the end counts as empty (0).
pub fn stack_or_empty(hotbar: &[u32], slot: usize) -> u32 {
    hotbar.get(slot).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_in_slot_is_none_past_the_hotbar() {
        assert_eq!(stack_in_slot(&[1, 16, 8], 1), Some(16));
        assert_eq!(stack_in_slot(&[1, 16, 8], 3), None, "three slots are 0, 1 and 2");
        assert_eq!(stack_in_slot(&[], 0), None);
    }

    #[test]
    fn stack_or_empty_falls_back() {
        assert_eq!(stack_or_empty(&[1, 16, 8], 0), 1);
        assert_eq!(stack_or_empty(&[1, 16, 8], 9), 0);
    }
}
