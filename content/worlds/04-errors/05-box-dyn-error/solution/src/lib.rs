use std::error::Error;

/// Parse "int,float" such as "3,4.5".
pub fn parse_pair(s: &str) -> Result<(i32, f64), Box<dyn Error>> {
    let (left, right) = s.split_once(',').ok_or("missing comma")?;
    let a = left.trim().parse::<i32>()?;
    let b = right.trim().parse::<f64>()?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_pair() {
        assert_eq!(parse_pair("3,4.5").unwrap(), (3, 4.5));
        assert_eq!(parse_pair(" 10 , 0.25 ").unwrap(), (10, 0.25));
    }

    #[test]
    fn missing_comma_is_an_error_with_message() {
        let err = parse_pair("34").unwrap_err();
        assert_eq!(err.to_string(), "missing comma");
    }

    #[test]
    fn bad_numbers_convert_into_the_box() {
        let int_err = parse_pair("x,1.0").unwrap_err();
        assert_eq!(int_err.to_string(), "invalid digit found in string");
        let float_err = parse_pair("1,y").unwrap_err();
        assert_eq!(float_err.to_string(), "invalid float literal");
    }
}
