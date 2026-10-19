/// validate_stack(32) -> Ok(32), validate_stack(100) -> Err("100 is over the stack limit of 64")
pub fn validate_stack(count: u32) -> Result<u32, String> {
    if count > 64 {
        Err(format!("{count} is over the stack limit of 64"))
    } else {
        Ok(count)
    }
}

/// Ok -> "stack of 32 is fine", Err -> "error: 100 is over the stack limit of 64"
pub fn describe_stack(count: u32) -> String {
    match validate_stack(count) {
        Ok(valid) => format!("stack of {valid} is fine"),
        Err(reason) => format!("error: {reason}"),
    }
}

/// "64" -> 64. The text is trusted server config, so a bad value is a bug: panic.
pub fn max_stack_from_config(text: &str) -> u32 {
    text.parse().expect("config max stack must be a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_divide_returns_result() {
        assert_eq!(validate_stack(32), Ok(32));
        assert_eq!(validate_stack(64), Ok(64), "exactly 64 is a full stack, not an error");
        assert_eq!(
            validate_stack(100),
            Err("100 is over the stack limit of 64".to_string())
        );
    }

    #[test]
    fn describe_division_matches_result() {
        assert_eq!(describe_stack(32), "stack of 32 is fine");
        assert_eq!(describe_stack(100), "error: 100 is over the stack limit of 64");
    }

    #[test]
    fn port_from_config_uses_expect() {
        assert_eq!(max_stack_from_config("64"), 64);
        assert_eq!(max_stack_from_config("16"), 16);
    }
}
