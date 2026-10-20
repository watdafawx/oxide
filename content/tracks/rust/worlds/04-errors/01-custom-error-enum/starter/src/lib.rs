use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    NotANumber(String),
    Negative(i64),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: write one message per variant: Empty => "input is empty",
        //       NotANumber(s) => "'<s>' is not a number", Negative(n) => "<n> is negative"
        todo!()
    }
}

impl std::error::Error for ParseError {}

/// Parse an age. Empty, non-numeric and negative input each get their own error.
pub fn parse_age(s: &str) -> Result<u32, ParseError> {
    // TODO: empty input (after trimming) -> Empty; text that is not a number -> NotANumber carrying the original text;
    //       below zero -> Negative(n); otherwise the age
    todo!()
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
