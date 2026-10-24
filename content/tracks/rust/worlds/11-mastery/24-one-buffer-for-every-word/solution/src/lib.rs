//! `tests/fast.rs` counts every allocation this function makes.
use std::collections::HashMap;

/// How often each word appears, ignoring case. A word is a run of letters and digits.
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut buf = String::new();
    for word in text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
        buf.clear();
        buf.extend(word.chars().flat_map(char::to_lowercase));
        match counts.get_mut(buf.as_str()) {
            Some(n) => *n += 1,
            None => {
                counts.insert(buf.clone(), 1);
            }
        }
    }
    counts
}
