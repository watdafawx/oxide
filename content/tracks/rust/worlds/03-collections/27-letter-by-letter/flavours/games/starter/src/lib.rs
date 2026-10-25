/// How many letters a player sees: "Zoë" -> 3 (its .len() is 4 bytes).
pub fn letter_count(name: &str) -> usize {
    // TODO: count the chars, not the bytes
    name.len()
}

/// An item id uses only a-z, 0-9, '_' and ':', and is not empty: "minecraft:oak_log" yes, "Oak Log" no.
pub fn is_valid_id(id: &str) -> bool {
    // TODO: not empty, and every char is an ASCII small letter, an ASCII digit, '_' or ':'
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_count_counts_letters_not_bytes() {
        assert_eq!(letter_count("Zoë"), 3, "ë is one letter, two bytes");
        assert_eq!(letter_count("Alex"), 4);
    }

    #[test]
    fn is_valid_id_checks_every_character() {
        assert!(is_valid_id("minecraft:oak_log"));
        assert!(!is_valid_id("Oak Log"), "capitals and spaces are not allowed");
        assert!(!is_valid_id("café"), "é is not a-z");
        assert!(!is_valid_id(""), "an empty id is not an id");
    }
}
