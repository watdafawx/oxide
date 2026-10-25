/// "Hello, <name>!" - reads the name, gives back new text.
pub fn greet(name: &str) -> String {
    // TODO: build "Hello, <name>!"
    todo!()
}

/// Copies borrowed text into a String the caller owns.
pub fn into_owned(text: &str) -> String {
    // TODO: make an owned String with the same letters
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
    fn into_owned_outlives_its_source() {
        let result = {
            let temp = String::from("short-lived");
            into_owned(&temp)
        };
        assert_eq!(result, "short-lived");
    }
}
