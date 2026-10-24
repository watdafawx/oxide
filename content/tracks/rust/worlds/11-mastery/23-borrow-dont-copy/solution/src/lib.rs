//! `tests/fast.rs` counts every allocation this function makes.

/// How many words are at least `min_len` bytes long.
pub fn count_long_words(text: &str, min_len: usize) -> usize {
    text.split_whitespace().filter(|w| w.len() >= min_len).count()
}
