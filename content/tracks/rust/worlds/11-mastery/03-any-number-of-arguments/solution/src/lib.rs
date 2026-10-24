/// `strings!["a", "b"]` is `vec!["a".to_string(), "b".to_string()]`.
#[macro_export]
macro_rules! strings {
    ($($s:expr),*) => {
        vec![$($s.to_string()),*]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn strings_converts_each_argument() {
        let words: Vec<String> = strings!["a", "b"];
        assert_eq!(words, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn strings_takes_any_count() {
        let one: Vec<String> = strings!["fox"];
        assert_eq!(one, vec!["fox".to_string()]);
        let five: Vec<String> = strings!["the", "quick", "brown", "fox", "jumps"];
        assert_eq!(five.len(), 5);
        assert_eq!(five[4], "jumps");
    }

    #[test]
    fn strings_takes_zero() {
        let none: Vec<String> = strings![];
        assert!(none.is_empty());
    }
}
