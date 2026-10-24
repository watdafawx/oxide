/// How many letters a person sees: "café" -> 4 (its .len() is 5 bytes).
pub fn letter_count(name: &str) -> usize {
    name.chars().count()
}

/// A product code uses only A-Z, 0-9 and '-', and is not empty: "MUG-04" yes, "mug 4" no.
pub fn is_valid_code(code: &str) -> bool {
    !code.is_empty() && code.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_count_counts_letters_not_bytes() {
        assert_eq!(letter_count("café"), 4, "é is one letter, two bytes");
        assert_eq!(letter_count("mug"), 3);
    }

    #[test]
    fn is_valid_code_checks_every_character() {
        assert!(is_valid_code("MUG-04"));
        assert!(!is_valid_code("mug 4"), "small letters and spaces are not allowed");
        assert!(!is_valid_code("CAFÉ"), "É is not A-Z");
        assert!(!is_valid_code(""), "an empty code is not a code");
    }
}
