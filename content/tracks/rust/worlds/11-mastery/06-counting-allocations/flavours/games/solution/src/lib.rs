/// Joins block names with a single space: `["stone", "dirt"]` -> `"stone dirt"`.
pub fn join_block_names(names: &[&str]) -> String {
    names.join(" ")
}

/// Removes consecutive duplicates from a sorted slice of chunk ids: `[1, 1, 2]` -> `[1, 2]`.
pub fn dedup_chunk_ids(input: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(input.len());
    for &x in input {
        if out.last() != Some(&x) {
            out.push(x);
        }
    }
    out
}
