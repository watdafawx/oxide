//! `tests/fast.rs` counts every allocation this function makes.
use std::collections::HashMap;

/// How often each block appears in a scan log, ignoring case. A block name is a run of letters
/// and digits.
pub fn tally_blocks(log: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut buf = String::new();
    for name in log.split(|c: char| !c.is_alphanumeric()).filter(|n| !n.is_empty()) {
        buf.clear();
        buf.extend(name.chars().flat_map(char::to_lowercase));
        match counts.get_mut(buf.as_str()) {
            Some(n) => *n += 1,
            None => {
                counts.insert(buf.clone(), 1);
            }
        }
    }
    counts
}
