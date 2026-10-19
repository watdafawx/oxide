/// Takes ownership of the name and says how long it is.
pub fn name_length(name: String) -> usize {
    name.len()
}

/// Takes ownership of the text and returns it in capitals.
pub fn shout(text: String) -> String {
    text.to_uppercase()
}

/// Returns the length of "ferris" AND the name itself.
pub fn length_and_name() -> (usize, String) {
    let name = String::from("ferris");
    let len = name_length(name.clone());
    (len, name)
}

/// Returns the shouted text AND the original.
pub fn shout_both() -> (String, String) {
    let original = String::from("hello");
    let loud = shout(original.clone());
    (loud, original)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_and_name_returns_both() {
        assert_eq!(length_and_name(), (6, "ferris".to_string()));
    }

    #[test]
    fn shout_both_returns_both() {
        assert_eq!(shout_both(), ("HELLO".to_string(), "hello".to_string()));
    }
}
