//! `tests/allocations.rs` counts every allocation these functions make.

/// `[0, 1, 4, 9, ...]`: the squares of `0..n`.
pub fn squares(n: usize) -> Vec<u64> {
    // TODO: right answer, but the Vec grows step by step. You know it will hold exactly n
    // items: start from a Vec with room for all of them.
    let mut out = Vec::new();
    for i in 0..n as u64 {
        out.push(i * i);
    }
    out
}

/// Joins words with single spaces: `["a", "b"]` -> `"a b"`.
pub fn join_words(words: &[&str]) -> String {
    // TODO: right answer, but the String grows step by step. Use the slice method that joins
    // with a separator: it adds up the final length first.
    let mut out = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}
