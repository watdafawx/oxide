/// Every unit in the shop: [12, 6, 1] -> 19
pub fn total_units(counts: &[u32]) -> u32 {
    // TODO: one chain that adds up every count
    0
}

/// How many counts are under `limit`? ([12, 2, 6, 1], 5) -> 2
pub fn count_low(counts: &[u32], limit: u32) -> usize {
    // TODO: one chain: keep the counts under `limit`, then count them
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_units_adds_every_count() {
        assert_eq!(total_units(&[12, 6, 1]), 19);
        assert_eq!(total_units(&[]), 0, "an empty shop holds 0 items");
    }

    #[test]
    fn count_low_counts_with_a_chain() {
        assert_eq!(count_low(&[12, 2, 6, 1], 5), 2);
        assert_eq!(count_low(&[12, 6], 5), 0);
    }
}
