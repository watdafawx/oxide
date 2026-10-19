use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    NotANumber(String),
    Negative(i64),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "input is empty"),
            ParseError::NotANumber(s) => write!(f, "'{s}' is not a number"),
            ParseError::Negative(n) => write!(f, "{n} is negative"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse an age. Empty, non-numeric and negative input each get their own error.
pub fn parse_age(s: &str) -> Result<u32, ParseError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    let n = trimmed
        .parse::<i64>()
        .map_err(|_| ParseError::NotANumber(s.to_string()))?;
    if n < 0 {
        return Err(ParseError::Negative(n));
    }
    Ok(n as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_gives_readable_messages() {
        assert_eq!(ParseError::Empty.to_string(), "input is empty");
        assert_eq!(ParseError::NotANumber("x".to_string()).to_string(), "'x' is not a number");
        assert_eq!(ParseError::Negative(-3).to_string(), "-3 is negative");
    }

    #[test]
    fn parse_age_reports_each_failure() {
        assert_eq!(parse_age(" 42 "), Ok(42));
        assert_eq!(parse_age("   "), Err(ParseError::Empty));
        assert_eq!(parse_age("old"), Err(ParseError::NotANumber("old".to_string())));
        assert_eq!(parse_age("-7"), Err(ParseError::Negative(-7)));
    }

    #[test]
    fn parse_error_works_as_dyn_error() {
        let err = parse_age("").unwrap_err();
        let dynamic: &dyn std::error::Error = &err;
        assert_eq!(dynamic.to_string(), "input is empty");
    }
}
