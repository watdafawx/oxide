pub mod probe;

use probe::{Counter, Name};

/// Counts how often each block name appears in a whitespace-separated log line.
pub fn count_blocks(log: &str, counter: &mut Counter) {
    for block in log.split_whitespace() {
        *counter.entry(block.to_string()).or_default() += 1;
    }
}

/// The longest name (the first one wins a tie), or None for an empty slice.
pub fn longest_name(names: &[Name]) -> Option<Name> {
    let mut best = names.first()?;
    for name in names {
        if name.0.len() > best.0.len() {
            best = name;
        }
    }
    Some(best.clone())
}

/// `[0, 1, 4, 9, ...]`: the chunk areas for sides `0..n`.
pub fn chunk_areas(n: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n as u64 {
        out.push(i * i);
    }
    out
}

/// How many block names are at least `min_len` bytes long.
pub fn count_long_names(log: &str, min_len: usize) -> usize {
    log.split_whitespace().filter(|w| w.len() >= min_len).count()
}
