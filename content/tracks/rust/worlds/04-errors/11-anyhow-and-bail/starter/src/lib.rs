use anyhow::{Result, bail};

/// Hours a task takes, at most a day. parse_hours(" 3 ") -> Ok(3)
pub fn parse_hours(text: &str) -> Result<u32> {
    // TODO: parse the trimmed text as a u32 with `?`; above 24, bail! with "<n> hours is more than a day"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hours_parse_and_bad_text_fails() {
        assert_eq!(parse_hours(" 3 ").unwrap(), 3);
        assert_eq!(parse_hours("24").unwrap(), 24, "a full day is still fine");
        let err = parse_hours("soon").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string", "the ParseIntError, converted by ?");
    }

    #[test]
    fn too_many_hours_bails() {
        let err = parse_hours("30").unwrap_err();
        assert_eq!(err.to_string(), "30 hours is more than a day");
    }
}
