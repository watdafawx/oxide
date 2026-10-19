/// Joins block names with a single space: `["stone", "dirt"]` -> `"stone dirt"`.
pub fn join_block_names(names: &[&str]) -> String {
    // TODO: build the string with as few allocations as possible
    let mut out = String::new();
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            out = out + " ";
        }
        out = out + name;
    }
    out
}

/// Removes consecutive duplicates from a sorted slice of chunk ids: `[1, 1, 2]` -> `[1, 2]`.
pub fn dedup_chunk_ids(input: &[i32]) -> Vec<i32> {
    // TODO: keep one copy of each id, allocating the output Vec only once
    let mut out = Vec::new();
    for &x in input {
        if out.last() != Some(&x) {
            out.push(x);
        }
    }
    out
}
