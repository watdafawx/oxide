/// " 12 " -> "12 boxes", "pizza" -> "'pizza' is not a quantity"
pub fn describe_quantity(text: &str) -> String {
    // TODO: match text.trim().parse::<u32>() { Ok(n) => ..., Err(_) => ... }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_number_is_read() {
        assert_eq!(describe_quantity("12"), "12 boxes");
        assert_eq!(describe_quantity(" 7 "), "7 boxes", "spaces around the number are fine");
    }

    #[test]
    fn junk_is_reported() {
        assert_eq!(describe_quantity("pizza"), "'pizza' is not a quantity");
        assert_eq!(describe_quantity("-3"), "'-3' is not a quantity", "a quantity is never negative");
        assert_eq!(describe_quantity(""), "'' is not a quantity");
    }
}
