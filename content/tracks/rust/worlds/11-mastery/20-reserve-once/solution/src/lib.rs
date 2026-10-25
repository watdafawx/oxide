//! `tests/allocations.rs` counts every allocation these functions make.

/// `[0, 1, 4, 9, ...]`: the squares of `0..n`.
pub fn squares(n: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n as u64 {
        out.push(i * i);
    }
    out
}

/// Joins words with single spaces: `["a", "b"]` -> `"a b"`.
pub fn join_words(words: &[&str]) -> String {
    words.join(" ")
}
