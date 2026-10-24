pub mod probe;

use probe::Counter;

/// Counts how often each whitespace-separated word appears.
pub fn word_count(text: &str, counter: &mut Counter) {
    // TODO: right answer, but a word seen before costs three operations (contains, get, insert).
    // Find or create the word's slot in one operation, then add 1 to the number in it.
    for word in text.split_whitespace() {
        if counter.contains(word) {
            let n = counter.get(word).unwrap_or(0);
            counter.insert(word.to_string(), n + 1);
        } else {
            counter.insert(word.to_string(), 1);
        }
    }
}
