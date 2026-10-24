/// `strings!["a", "b"]` is `vec!["a".to_string(), "b".to_string()]`. A comma after the last
/// word is fine.
#[macro_export]
macro_rules! strings {
    // TODO: also accept one optional comma after the last argument
    ($($s:expr),*) => {
        vec![$($s.to_string()),*]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn strings_accepts_a_trailing_comma() {
        let words: Vec<String> = strings![
            "the",
            "quick",
        ];
        assert_eq!(words, vec!["the".to_string(), "quick".to_string()]);
    }

    #[test]
    fn strings_still_works_without_one() {
        let words: Vec<String> = strings!["fox", "dog"];
        assert_eq!(words.len(), 2);
        let none: Vec<String> = strings![];
        assert!(none.is_empty());
    }
}
