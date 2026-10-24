use std::thread;

/// Adds up all file sizes with two scoped threads: one for each half of the slice.
pub fn parallel_sum(sizes: &[u64]) -> u64 {
    let mid = sizes.len() / 2;
    // TODO: cut sizes into &sizes[..mid] and &sizes[mid..]; inside thread::scope,
    // sum each half on its own s.spawn thread and return both sums added together
    let _ = mid;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_sum_borrows_both_halves() {
        let sizes = vec![100, 250, 650, 4_000, 1];
        assert_eq!(parallel_sum(&sizes), 5_001);
        assert_eq!(sizes.len(), 5, "the sizes are only borrowed");
        assert_eq!(parallel_sum(&[]), 0);
        assert_eq!(parallel_sum(&[7]), 7);
    }
}
