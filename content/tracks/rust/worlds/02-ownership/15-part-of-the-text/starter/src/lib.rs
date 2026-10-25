/// Everything before the first space; the whole text if there is no space.
pub fn first_word(text: &str) -> &str {
    // TODO: find the first space and borrow the part before it
    todo!()
}

/// Everything after the first space; "" if there is no space.
pub fn after_first_word(text: &str) -> &str {
    // TODO: find the first space and borrow the part after it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_stops_at_space() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
        let text = String::from("borrowed words");
        assert_eq!(first_word(&text).as_ptr(), text.as_ptr(), "slice the input, don't copy it");
    }

    #[test]
    fn after_first_word_is_the_rest() {
        assert_eq!(after_first_word("hello big world"), "big world");
        assert_eq!(after_first_word("single"), "");
    }
}
