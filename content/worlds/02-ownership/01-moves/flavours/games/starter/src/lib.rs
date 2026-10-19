/// The log keeps the chat line for good and reports how long it was.
pub fn log_length(line: String) -> usize {
    line.len()
}

/// Returns the length of the chat line AND the line itself.
pub fn length_and_name() -> (usize, String) {
    let line = String::from("<Ferris> hello");
    // TODO: this does not compile. `line` moves into log_length, so it can't be returned after.
    let len = log_length(line);
    (len, line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_and_name_returns_both() {
        assert_eq!(length_and_name(), (14, "<Ferris> hello".to_string()));
    }
}
