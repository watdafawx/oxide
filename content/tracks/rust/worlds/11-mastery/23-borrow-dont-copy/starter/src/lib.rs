//! `tests/fast.rs` counts every allocation this function makes.

/// How many words are at least `min_len` bytes long.
pub fn count_long_words(text: &str, min_len: usize) -> usize {
    // TODO: right answer, but every word is copied into a new String and the Strings into a new
    // Vec. Count straight from the &str pieces that split_whitespace hands out.
    let words: Vec<String> = text.split_whitespace().map(String::from).collect();
    words.iter().filter(|w| w.len() >= min_len).count()
}
