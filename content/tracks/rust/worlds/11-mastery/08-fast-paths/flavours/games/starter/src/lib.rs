pub mod probe;

use probe::{Counter, Name};

/// Counts how often each block name appears in a whitespace-separated log line.
pub fn count_blocks(log: &str, counter: &mut Counter) {
    // TODO: one Counter operation per block: use entry(..).or_default()
    for block in log.split_whitespace() {
        if counter.contains(block) {
            let n = counter.get(block).unwrap_or(0);
            counter.insert(block.to_string(), n + 1);
        } else {
            counter.insert(block.to_string(), 1);
        }
    }
}

/// The longest name (the first one wins a tie), or None for an empty slice.
pub fn longest_name(names: &[Name]) -> Option<Name> {
    // TODO: compare by reference, clone only the winner
    let mut best = names.first()?.clone();
    for name in names {
        if name.0.len() > best.0.len() {
            best = name.clone();
        }
    }
    Some(best)
}

/// `[0, 1, 4, 9, ...]`: the chunk areas for sides `0..n`.
pub fn chunk_areas(n: usize) -> Vec<u64> {
    // TODO: reserve the capacity first
    let mut out = Vec::new();
    for i in 0..n as u64 {
        out.push(i * i);
    }
    out
}

/// How many block names are at least `min_len` bytes long.
pub fn count_long_names(log: &str, min_len: usize) -> usize {
    // TODO: count with an iterator chain, no Vec or String
    let words: Vec<String> = log.split_whitespace().map(|w| w.to_string()).collect();
    words.iter().filter(|w| w.len() >= min_len).count()
}
