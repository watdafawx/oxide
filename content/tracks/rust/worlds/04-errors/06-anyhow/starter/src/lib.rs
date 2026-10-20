use anyhow::{Context, Result, bail};

/// Parse a port number. A failure should say "port must be a number".
pub fn parse_port(s: &str) -> Result<u16> {
    // TODO: parse the trimmed text as a port; a failure gets the context "port must be a number"
    todo!()
}

/// Accept 0 to 100 inclusive, otherwise fail with "percent out of range: <n>".
pub fn check_percent(n: i32) -> Result<i32> {
    // TODO: fail with "percent out of range: {n}" when n is outside 0 to 100, otherwise return n
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_port_parses() {
        assert_eq!(parse_port(" 8080 ").unwrap(), 8080);
    }

    #[test]
    fn bad_port_has_context_message() {
        let err = parse_port("http").unwrap_err();
        assert_eq!(err.to_string(), "port must be a number");
    }

    #[test]
    fn context_keeps_the_original_cause() {
        let err = parse_port("http").unwrap_err();
        assert_eq!(err.root_cause().to_string(), "invalid digit found in string");
    }

    #[test]
    fn check_percent_bails_when_out_of_range() {
        assert_eq!(check_percent(50).unwrap(), 50);
        assert_eq!(check_percent(0).unwrap(), 0);
        assert_eq!(check_percent(100).unwrap(), 100);
        assert_eq!(check_percent(150).unwrap_err().to_string(), "percent out of range: 150");
        assert_eq!(check_percent(-1).unwrap_err().to_string(), "percent out of range: -1");
    }
}
