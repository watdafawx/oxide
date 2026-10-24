//! `tests/fast.rs` counts every allocation this function makes.

/// How many block names in a scan line are at least `min_len` bytes long.
pub fn count_long_names(scan: &str, min_len: usize) -> usize {
    scan.split_whitespace().filter(|w| w.len() >= min_len).count()
}
