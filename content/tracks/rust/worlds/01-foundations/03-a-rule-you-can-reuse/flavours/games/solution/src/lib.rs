/// One log makes four planks.
pub fn planks(logs: u32) -> u32 {
    logs * 4
}

/// One plank makes two sticks.
pub fn sticks(planks: u32) -> u32 {
    planks * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planks_for_any_log_count() {
        assert_eq!(planks(12), 48);
        assert_eq!(planks(7), 28);
        assert_eq!(planks(0), 0);
    }

    #[test]
    fn sticks_for_any_plank_count() {
        assert_eq!(sticks(48), 96);
        assert_eq!(sticks(1), 2);
    }
}
