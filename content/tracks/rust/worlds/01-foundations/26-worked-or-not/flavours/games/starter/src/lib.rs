/// Accept a stack of up to 64 items.
/// validate_stack(32) -> Ok(32), validate_stack(100) -> Err("100 is over the stack limit of 64")
pub fn validate_stack(count: u32) -> Result<u32, String> {
    // TODO: if count > 64 { Err(format!(..)) } else { Ok(count) }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_valid_stack_is_ok() {
        assert_eq!(validate_stack(32), Ok(32));
        assert_eq!(validate_stack(64), Ok(64), "exactly 64 is a full stack, not an error");
    }

    #[test]
    fn an_oversized_stack_is_err() {
        assert_eq!(
            validate_stack(100),
            Err("100 is over the stack limit of 64".to_string())
        );
    }
}
