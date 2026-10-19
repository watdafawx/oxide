/// Joins words with a single space: `["a", "b"]` -> `"a b"`.
pub fn join_words(words: &[&str]) -> String {
    words.join(" ")
}

/// Removes consecutive duplicates from a sorted slice: `[1, 1, 2]` -> `[1, 2]`.
pub fn dedup_sorted(input: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(input.len());
    for &x in input {
        if out.last() != Some(&x) {
            out.push(x);
        }
    }
    out
}
