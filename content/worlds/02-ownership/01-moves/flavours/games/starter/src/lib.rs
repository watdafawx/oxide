/// The log keeps the chat line for good and reports how long it was.
pub fn log_length(line: String) -> usize {
    line.len()
}

/// Broadcasts the chat line in capitals.
pub fn broadcast(line: String) -> String {
    line.to_uppercase()
}

/// Returns the length of the chat line AND the line itself.
pub fn length_and_name() -> (usize, String) {
    let line = String::from("<Ferris> hello");
    // TODO: this does not compile. `line` moves into log_length, so it can't be returned after.
    let len = log_length(line);
    (len, line)
}

/// Returns the broadcast line AND the original.
pub fn shout_both() -> (String, String) {
    let original = String::from("gg everyone");
    // TODO: same problem, same fix.
    let loud = broadcast(original);
    (loud, original)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_and_name_returns_both() {
        assert_eq!(length_and_name(), (14, "<Ferris> hello".to_string()));
    }

    #[test]
    fn shout_both_returns_both() {
        assert_eq!(shout_both(), ("GG EVERYONE".to_string(), "gg everyone".to_string()));
    }
}
