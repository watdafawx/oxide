pub mod probe;

use probe::Counter;

/// Counts how often each whitespace-separated word appears.
pub fn word_count(text: &str, counter: &mut Counter) {
    for word in text.split_whitespace() {
        *counter.entry(word.to_string()).or_default() += 1;
    }
}
