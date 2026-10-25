/// The log keeps the chat line for good and reports how long it was.
pub fn log_length(line: String) -> usize {
    line.len()
}

/// The length of the chat line AND the line itself.
pub fn length_and_line() -> (usize, String) {
    let line = String::from("<Ferris> hello");
    // TODO: this does not build: `line` moves into log_length, so the tuple can't use it after.
    // Give log_length its own copy.
    let len = log_length(line);
    (len, line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_and_line_returns_both() {
        assert_eq!(length_and_line(), (14, "<Ferris> hello".to_string()));
    }
}
