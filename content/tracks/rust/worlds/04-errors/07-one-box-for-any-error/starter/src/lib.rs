use std::error::Error;

/// Read an estimate like "3,1.5": priority 3, 1.5 hours.
pub fn parse_estimate(text: &str) -> Result<(u32, f64), Box<dyn Error>> {
    // TODO: cut the text at the comma (no comma -> the error "missing comma"),
    //       then read the left half as a whole-number priority and the right half as hours, trimming each
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_priority_and_hours() {
        assert_eq!(parse_estimate("3,1.5").unwrap(), (3, 1.5));
        assert_eq!(parse_estimate(" 2 , 0.25 \n").unwrap(), (2, 0.25), "trim both halves");
    }

    #[test]
    fn missing_comma_says_so() {
        let err = parse_estimate("3 1.5").unwrap_err();
        assert_eq!(err.to_string(), "missing comma");
    }

    #[test]
    fn bad_numbers_fit_in_the_box() {
        let err = parse_estimate("high,1.5").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string", "a ParseIntError in the box");
        let err = parse_estimate("3,soon").unwrap_err();
        assert_eq!(err.to_string(), "invalid float literal", "a ParseFloatError in the box");
    }
}
