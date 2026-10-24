pub mod probe;

use probe::{Counter, Name};

/// Counts how often each whitespace-separated word appears.
pub fn word_count(text: &str, counter: &mut Counter) {
    // TODO: three operations per repeated word is too many. Do it in one with the entry API:
    // find or create the word's slot, then add 1 to it.
    for word in text.split_whitespace() {
        if counter.contains(word) {
            let n = counter.get(word).unwrap_or(0);
            counter.insert(word.to_string(), n + 1);
        } else {
            counter.insert(word.to_string(), 1);
        }
    }
}

/// The longest name (the first one wins a tie), or None for an empty slice.
pub fn longest(names: &[Name]) -> Option<Name> {
    // TODO: every `clone()` here is counted. Hold a `&Name` while searching, clone once at the end.
    let mut best = names.first()?.clone();
    for name in names {
        if name.0.len() > best.0.len() {
            best = name.clone();
        }
    }
    Some(best)
}

/// `[0, 1, 4, 9, ...]`: the first `n` squares.
pub fn squares(n: usize) -> Vec<u64> {
    // TODO: you know the length up front. Start from a Vec with room for n items.
    let mut out = Vec::new();
    for i in 0..n as u64 {
        out.push(i * i);
    }
    out
}

/// How many words are at least `min_len` bytes long.
pub fn count_long_words(text: &str, min_len: usize) -> usize {
    // TODO: copying every word into a String is not needed. Work on the `&str`s directly.
    let words: Vec<String> = text.split_whitespace().map(String::from).collect();
    words.iter().filter(|w| w.len() >= min_len).count()
}
