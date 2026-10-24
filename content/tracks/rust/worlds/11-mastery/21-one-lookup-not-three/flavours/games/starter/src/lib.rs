pub mod probe;

use probe::Counter;

/// Counts how often each block name appears in a whitespace-separated scan line.
pub fn count_blocks(scan: &str, counter: &mut Counter) {
    // TODO: right answer, but a block seen before costs three operations (contains, get, insert).
    // Find or create the block's slot in one operation, then add 1 to the number in it.
    for block in scan.split_whitespace() {
        if counter.contains(block) {
            let n = counter.get(block).unwrap_or(0);
            counter.insert(block.to_string(), n + 1);
        } else {
            counter.insert(block.to_string(), 1);
        }
    }
}
