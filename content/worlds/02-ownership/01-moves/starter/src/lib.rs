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
    // TODO: this does not compile. `name` moves into name_length, so it can't be returned after.
    let len = name_length(name);
    (len, name)
}

/// Returns the shouted text AND the original.
pub fn shout_both() -> (String, String) {
    let original = String::from("hello");
    // TODO: same problem, same fix.
    let loud = shout(original);
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
