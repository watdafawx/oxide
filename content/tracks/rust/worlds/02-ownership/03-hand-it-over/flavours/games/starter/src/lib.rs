/// One entry in the chat log: it owns the line and remembers how wide it is.
#[derive(Debug, PartialEq)]
pub struct ChatEntry {
    pub text: String,
    pub width: usize,
}

/// Puts a chat line into the log.
pub fn log_line(line: String) -> ChatEntry {
    // TODO: this does not build: `line` moves into the entry, then `line.len()` uses it again.
    // Work out the width before the line moves.
    ChatEntry { text: line, width: line.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_line_keeps_text_and_width() {
        let entry = log_line(String::from("<Steve> hi"));
        assert_eq!(entry.text, "<Steve> hi");
        assert_eq!(entry.width, 10, "\"<Steve> hi\" is 10 characters wide");
    }
}
