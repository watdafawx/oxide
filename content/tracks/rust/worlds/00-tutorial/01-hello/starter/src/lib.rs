/// Build the greeting for `name`: `Hello, Ferris!`
pub fn greeting(name: &str) -> String {
    // TODO: use format! to build the String, like the lesson's example
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greeting("Ferris"), "Hello, Ferris!");
        assert_eq!(greeting("you"), "Hello, you!");
    }
}
