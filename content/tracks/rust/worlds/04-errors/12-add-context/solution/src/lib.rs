use anyhow::{Context, Result};

/// A priority field. A bad one says "priority must be a number" on top of the parse error.
pub fn parse_priority(text: &str) -> Result<u32> {
    text.trim().parse::<u32>().context("priority must be a number")
}

/// The same, with the file's line number on top: "line 3".
pub fn priority_on_line(line: u32, text: &str) -> Result<u32> {
    parse_priority(text).with_context(|| format!("line {line}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_sits_on_top_of_the_root_cause() {
        assert_eq!(parse_priority(" 2 ").unwrap(), 2);
        let err = parse_priority("high").unwrap_err();
        assert_eq!(err.to_string(), "priority must be a number");
        assert_eq!(err.root_cause().to_string(), "invalid digit found in string");
    }

    #[test]
    fn with_context_names_the_line() {
        assert_eq!(priority_on_line(1, "4").unwrap(), 4);
        let err = priority_on_line(3, "high").unwrap_err();
        assert_eq!(err.to_string(), "line 3");
        assert_eq!(format!("{err:#}"), "line 3: priority must be a number: invalid digit found in string");
    }
}
