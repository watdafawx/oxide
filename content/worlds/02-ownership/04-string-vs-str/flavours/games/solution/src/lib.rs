/// "Welcome, <name>!" - reads the name, returns a new chat line.
pub fn welcome(name: &str) -> String {
    format!("Welcome, {name}!")
}

/// Copies borrowed text into a chat line the caller owns.
pub fn chat_line(text: &str) -> String {
    text.to_string()
}

/// "owner" -> 3, "op" -> 2, anything else -> 1.
pub fn permission_level(role: String) -> u32 {
    match role.as_str() {
        "owner" => 3,
        "op" => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_takes_literals_and_strings() {
        let joined = String::from("Ferris");
        assert_eq!(welcome("Ana"), "Welcome, Ana!");
        assert_eq!(welcome(&joined), "Welcome, Ferris!");
    }

    #[test]
    fn into_owned_outlives_source() {
        let result = {
            let typed = String::from("brb");
            chat_line(&typed)
        };
        assert_eq!(result, "brb");
    }

    #[test]
    fn rank_owned_matches_on_as_str() {
        assert_eq!(permission_level("owner".to_string()), 3);
        assert_eq!(permission_level("op".to_string()), 2);
        assert_eq!(permission_level("guest".to_string()), 1);
    }
}
