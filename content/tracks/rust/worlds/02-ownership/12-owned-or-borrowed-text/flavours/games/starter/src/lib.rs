/// "Welcome, <name>!" - reads the name, gives back a new chat line.
pub fn welcome(name: &str) -> String {
    // TODO: build "Welcome, <name>!"
    todo!()
}

/// Copies borrowed text into a chat line the log owns.
pub fn chat_line(text: &str) -> String {
    // TODO: make an owned String with the same letters
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
}
