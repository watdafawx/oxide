//! `tests/fast.rs` counts every allocation this function makes.
use std::collections::HashMap;

/// How often each word appears, ignoring case. A word is a run of letters and digits.
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    // TODO: right answer, but every word allocates a new lowercase String, even a word seen
    // before. Keep one String buffer: for each word, clear it and fill it with the lowercased
    // letters. Look the buffer up as a &str, and copy it into a new key only for a new word.
    for word in text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
        *counts.entry(word.to_lowercase()).or_default() += 1;
    }
    counts
}
