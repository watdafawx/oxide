use std::num::ParseIntError;

/// Parse `s` as a number and double it. Bad input is an error, not a panic.
pub fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}

/// The first character of `s`, uppercased. None for an empty string.
pub fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    c.to_uppercase().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_double_doubles_a_number() {
        assert_eq!(parse_and_double("21"), Ok(42));
        assert_eq!(parse_and_double("-4"), Ok(-8));
    }

    #[test]
    fn parse_and_double_passes_the_error_up() {
        assert!(parse_and_double("abc").is_err(), "\"abc\" is not a number");
        assert!(parse_and_double("").is_err(), "empty string is not a number");
    }

    #[test]
    fn first_char_upper_uses_question_mark_on_option() {
        assert_eq!(first_char_upper("hello"), Some('H'));
        assert_eq!(first_char_upper(""), None, "empty string has no first char");
    }
}
