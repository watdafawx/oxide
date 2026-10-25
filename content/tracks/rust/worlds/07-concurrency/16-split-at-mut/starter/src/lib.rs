use std::thread;

/// Turns every size from KiB into bytes (times 1024), in place.
/// Two scoped threads: one for each half of the slice.
pub fn kib_to_bytes(sizes: &mut [u64]) {
    let mid = sizes.len() / 2;
    // TODO: this does not build (E0499). Cut sizes into two &mut halves with one call.
    let left = &mut sizes[..mid];
    let right = &mut sizes[mid..];
    thread::scope(|s| {
        s.spawn(|| {
            for size in left.iter_mut() {
                *size *= 1024;
            }
        });
        s.spawn(|| {
            for size in right.iter_mut() {
                *size *= 1024;
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kib_to_bytes_changes_both_halves() {
        let mut sizes = vec![1, 2, 3, 4, 5];
        kib_to_bytes(&mut sizes);
        assert_eq!(sizes, vec![1024, 2048, 3072, 4096, 5120]);
        let mut empty: Vec<u64> = Vec::new();
        kib_to_bytes(&mut empty);
        assert!(empty.is_empty());
    }
}
