use std::thread;

/// Adds up all jar sizes with two scoped threads: one for each half of the slice.
pub fn parallel_sum(sizes: &[u64]) -> u64 {
    let mid = sizes.len() / 2;
    let left = &sizes[..mid];
    let right = &sizes[mid..];
    thread::scope(|s| {
        let a = s.spawn(|| left.iter().sum::<u64>());
        let b = s.spawn(|| right.iter().sum::<u64>());
        a.join().unwrap() + b.join().unwrap()
    })
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
