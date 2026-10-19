use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum MyError {
    Parse(ParseIntError),
    TooBig(i32),
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

/// Parse a number that must be 100 or less.
pub fn parse_small(s: &str) -> Result<i32, MyError> {
    let n: i32 = s.parse()?;
    if n > 100 { Err(MyError::TooBig(n)) } else { Ok(n) }
}

/// Add up every input. Stop at the first error.
pub fn sum_all(inputs: &[&str]) -> Result<i32, MyError> {
    let mut total = 0;
    for input in inputs {
        total += parse_small(input)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_wraps_the_parse_error() {
        let parse_err = "x".parse::<i32>().unwrap_err();
        assert_eq!(MyError::from(parse_err.clone()), MyError::Parse(parse_err));
    }

    #[test]
    fn sum_all_stops_at_first_bad_input() {
        assert_eq!(sum_all(&["1", "2", "3"]), Ok(6));
        assert_eq!(sum_all(&["1", "500", "x"]), Err(MyError::TooBig(500)));
        let parse_err = "x".parse::<i32>().unwrap_err();
        assert_eq!(sum_all(&["1", "x"]), Err(MyError::Parse(parse_err)));
    }
}
