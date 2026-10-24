/// How many letters a person sees: "café" -> 4 (its .len() is 5 bytes).
pub fn letter_count(name: &str) -> usize {
    // TODO: count the chars, not the bytes
    name.len()
}

/// A product code uses only A-Z, 0-9 and '-', and is not empty: "MUG-04" yes, "mug 4" no.
pub fn is_valid_code(code: &str) -> bool {
    // TODO: not empty, and every char is an ASCII capital, an ASCII digit or '-'
    false
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
