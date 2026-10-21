/// " 12 " -> "12 items", "pizza" -> "'pizza' is not a count"
pub fn describe_count(text: &str) -> String {
    match text.trim().parse::<u32>() {
        Ok(n) => format!("{n} items"),
        Err(_) => format!("'{text}' is not a count"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_number_is_read() {
        assert_eq!(describe_count("12"), "12 items");
        assert_eq!(describe_count(" 7 "), "7 items", "spaces around the number are fine");
    }

    #[test]
    fn junk_is_reported() {
        assert_eq!(describe_count("pizza"), "'pizza' is not a count");
        assert_eq!(describe_count("-3"), "'-3' is not a count", "a count is never negative");
        assert_eq!(describe_count(""), "'' is not a count");
    }
}
