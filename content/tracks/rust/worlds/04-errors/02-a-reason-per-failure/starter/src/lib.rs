/// Every way a typed priority can be wrong.
/// (`#[derive(Debug, PartialEq)]` lets the tests compare and print errors. Two quests from now explain that line.)
#[derive(Debug, PartialEq)]
pub enum PriorityError {
    Empty,
    NotANumber(String),
    TooHigh(u32),
}

/// Read a priority from 1 to 5. parse_priority(" 3 ") -> Ok(3)
pub fn parse_priority(text: &str) -> Result<u32, PriorityError> {
    // TODO: trim the text. Nothing left (`is_empty()`) -> Empty; not a whole number -> NotANumber with the
    //       trimmed text; above 5 -> TooHigh with the number; otherwise the number
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_good_priority_is_ok() {
        assert_eq!(parse_priority(" 3 "), Ok(3));
        assert_eq!(parse_priority("5"), Ok(5), "5 is the highest priority, still fine");
    }

    #[test]
    fn each_failure_has_its_own_variant() {
        assert_eq!(parse_priority("   "), Err(PriorityError::Empty));
        assert_eq!(parse_priority("high"), Err(PriorityError::NotANumber("high".to_string())));
        assert_eq!(parse_priority("9"), Err(PriorityError::TooHigh(9)));
    }
}
