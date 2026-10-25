//! `tests/allocations.rs` counts every allocation these functions make.

/// `[0, 1, 4, 9, ...]`: the areas of square chunks with sides `0..n`.
pub fn chunk_areas(n: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n as u64 {
        out.push(i * i);
    }
    out
}

/// Joins block names with single spaces: `["stone", "dirt"]` -> `"stone dirt"`.
pub fn join_block_names(names: &[&str]) -> String {
    names.join(" ")
}
