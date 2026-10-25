pub mod probe;

use probe::Counter;

/// Counts how often each block name appears in a whitespace-separated scan line.
pub fn count_blocks(scan: &str, counter: &mut Counter) {
    for block in scan.split_whitespace() {
        *counter.entry(block.to_string()).or_default() += 1;
    }
}
