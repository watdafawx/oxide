//! `tests/fast.rs` counts every allocation this function makes.
use std::collections::HashMap;

/// How often each block appears in a scan log, ignoring case. A block name is a run of letters
/// and digits.
pub fn tally_blocks(log: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    // TODO: right answer, but every block name allocates a new lowercase String, even a name seen
    // before. Keep one String buffer: for each name, clear it and fill it with the lowercased
    // letters. Look the buffer up as a &str, and copy it into a new key only for a new name.
    for name in log.split(|c: char| !c.is_alphanumeric()).filter(|n| !n.is_empty()) {
        *counts.entry(name.to_lowercase()).or_default() += 1;
    }
    counts
}
