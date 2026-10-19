/// "Hello, <name>!" - reads the name, returns new text.
pub fn greet(name: &str) -> String {
    // TODO: build "Hello, <name>!"
    todo!()
}

/// Copies borrowed text into a String the caller owns.
pub fn into_owned(text: &str) -> String {
    // TODO: make an owned String from text
    todo!()
}

/// "admin" -> 3, "mod" -> 2, anything else -> 1.
pub fn rank_owned(name: String) -> u32 {
    // TODO: match on name.as_str()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_takes_literals_and_strings() {
        let owned = String::from("Ferris");
        assert_eq!(greet("Ana"), "Hello, Ana!");
        assert_eq!(greet(&owned), "Hello, Ferris!");
    }

    #[test]
    fn into_owned_outlives_source() {
        let result = {
            let temp = String::from("short-lived");
            into_owned(&temp)
        };
        assert_eq!(result, "short-lived");
    }

    #[test]
    fn rank_owned_matches_on_as_str() {
        assert_eq!(rank_owned("admin".to_string()), 3);
        assert_eq!(rank_owned("mod".to_string()), 2);
        assert_eq!(rank_owned("guest".to_string()), 1);
    }
}
