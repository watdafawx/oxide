/// "Welcome, <name>!" - reads the name, returns a new chat line.
pub fn welcome(name: &str) -> String {
    // TODO: build "Welcome, <name>!"
    todo!()
}

/// Copies borrowed text into a chat line the caller owns.
pub fn chat_line(text: &str) -> String {
    // TODO: make an owned String from text
    todo!()
}

/// "owner" -> 3, "op" -> 2, anything else -> 1.
pub fn permission_level(role: String) -> u32 {
    // TODO: match on role.as_str()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_takes_literals_and_strings() {
        let joined = String::from("Ferris");
        assert_eq!(welcome("Ana"), "Welcome, Ana!");
        assert_eq!(welcome(&joined), "Welcome, Ferris!");
    }

    #[test]
    fn chat_line_outlives_typed_text() {
        let result = {
            let typed = String::from("brb");
            chat_line(&typed)
        };
        assert_eq!(result, "brb");
    }

    #[test]
    fn permission_level_matches_role_text() {
        assert_eq!(permission_level("owner".to_string()), 3);
        assert_eq!(permission_level("op".to_string()), 2);
        assert_eq!(permission_level("guest".to_string()), 1);
    }
}
