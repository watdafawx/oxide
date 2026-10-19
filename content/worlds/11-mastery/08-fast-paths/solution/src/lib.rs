pub mod probe;

use probe::{Counter, Name};

/// Counts how often each whitespace-separated word appears.
pub fn word_count(text: &str, counter: &mut Counter) {
    for word in text.split_whitespace() {
        *counter.entry(word.to_string()).or_default() += 1;
    }
}

/// The longest name (the first one wins a tie), or None for an empty slice.
pub fn longest(names: &[Name]) -> Option<Name> {
    let mut best = names.first()?;
    for name in names {
        if name.0.len() > best.0.len() {
            best = name;
        }
    }
    Some(best.clone())
}

/// `[0, 1, 4, 9, ...]`: the first `n` squares.
pub fn squares(n: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n as u64 {
        out.push(i * i);
    }
    out
}

/// How many words are at least `min_len` bytes long.
pub fn count_long_words(text: &str, min_len: usize) -> usize {
    text.split_whitespace().filter(|w| w.len() >= min_len).count()
}
