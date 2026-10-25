//! `tests/fast.rs` counts every allocation this function makes.

/// How many block names in a scan line are at least `min_len` bytes long.
pub fn count_long_names(scan: &str, min_len: usize) -> usize {
    // TODO: right answer, but every name is copied into a new String and the Strings into a new
    // Vec. Count straight from the &str pieces that split_whitespace hands out.
    let words: Vec<String> = scan.split_whitespace().map(String::from).collect();
    words.iter().filter(|w| w.len() >= min_len).count()
}
