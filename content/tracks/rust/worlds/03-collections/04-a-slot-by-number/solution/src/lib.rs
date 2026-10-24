/// The count on shelf number `shelf` (0 is the first). None if there is no such shelf.
pub fn count_on_shelf(counts: &[u32], shelf: usize) -> Option<u32> {
    counts.get(shelf).copied()
}

/// The same, but a shelf that does not exist counts as 0.
pub fn count_or_zero(counts: &[u32], shelf: usize) -> u32 {
    counts.get(shelf).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_on_shelf_is_none_past_the_end() {
        assert_eq!(count_on_shelf(&[12, 6, 1], 1), Some(6));
        assert_eq!(count_on_shelf(&[12, 6, 1], 3), None, "three shelves are 0, 1 and 2");
        assert_eq!(count_on_shelf(&[], 0), None);
    }

    #[test]
    fn count_or_zero_falls_back() {
        assert_eq!(count_or_zero(&[12, 6, 1], 0), 12);
        assert_eq!(count_or_zero(&[12, 6, 1], 9), 0);
    }
}
