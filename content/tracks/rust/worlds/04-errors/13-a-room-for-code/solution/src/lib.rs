mod store {
    /// The list holds at most this many tasks. Private: only code in `store` sees it.
    const MAX_TASKS: u32 = 50;

    pub fn room_left(count: u32) -> u32 {
        MAX_TASKS.saturating_sub(count)
    }
}

/// Can one more task go in, when the list already has `count`?
pub fn can_add(count: u32) -> bool {
    store::room_left(count) > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_left_is_open_to_callers() {
        assert_eq!(store::room_left(48), 2);
        assert_eq!(store::room_left(60), 0);
    }

    #[test]
    fn can_add_asks_the_store() {
        assert!(can_add(10));
        assert!(can_add(49), "one slot left");
        assert!(!can_add(50), "the list is full");
    }
}
