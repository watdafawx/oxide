use std::error::Error;

/// Read an estimate like "3,1.5": priority 3, 1.5 hours.
pub fn parse_estimate(text: &str) -> Result<(u32, f64), Box<dyn Error>> {
    let (left, right) = match text.split_once(',') {
        Some(pair) => pair,
        None => return Err("missing comma".into()),
    };
    let priority: u32 = left.trim().parse()?;
    let hours: f64 = right.trim().parse()?;
    Ok((priority, hours))
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
