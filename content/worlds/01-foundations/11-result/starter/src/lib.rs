/// safe_divide(8, 2) -> Ok(4), safe_divide(1, 0) -> Err("cannot divide by zero")
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    // TODO: Err(...) if b == 0, otherwise Ok(a / b)
    todo!()
}

/// Ok -> "8 / 2 = 4", Err -> "error: cannot divide by zero"
pub fn describe_division(a: i32, b: i32) -> String {
    // TODO: match safe_divide(a, b) { Ok(..) => format!(..), Err(..) => format!(..) }
    todo!()
}

/// "8080" -> 8080. The text is trusted config, so a bad value is a bug: panic.
pub fn port_from_config(text: &str) -> u16 {
    // TODO: text.parse() gives a Result; call .expect("config port must be a number")
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_divide_returns_result() {
        assert_eq!(safe_divide(8, 2), Ok(4));
        assert_eq!(safe_divide(1, 0), Err("cannot divide by zero".to_string()));
    }

    #[test]
    fn describe_division_matches_result() {
        assert_eq!(describe_division(8, 2), "8 / 2 = 4");
        assert_eq!(describe_division(1, 0), "error: cannot divide by zero");
    }

    #[test]
    fn port_from_config_uses_expect() {
        assert_eq!(port_from_config("8080"), 8080);
        assert_eq!(port_from_config("22"), 22);
    }
}
