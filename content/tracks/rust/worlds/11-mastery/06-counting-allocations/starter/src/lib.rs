/// Joins words with a single space: `["a", "b"]` -> `"a b"`.
pub fn join_words(words: &[&str]) -> String {
    // TODO: this works but reallocates as it grows. Reserve once, or use `words.join(" ")`.
    let mut out = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}

/// Removes consecutive duplicates from a sorted slice: `[1, 1, 2]` -> `[1, 2]`.
pub fn dedup_sorted(input: &[i32]) -> Vec<i32> {
    // TODO: this works but the Vec grows step by step. Allocate the room up front: `Vec::with_capacity(input.len())` instead of `Vec::new()`.
    let mut out = Vec::new();
    for &x in input {
        if out.last() != Some(&x) {
            out.push(x);
        }
    }
    out
}
