/// The chat line everyone sees when `player` joins: `Welcome to the server, Ferris!`
pub fn join_message(player: &str) -> String {
    // TODO: use format! to build the String
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(join_message("Ferris"), "Welcome to the server, Ferris!");
        assert_eq!(join_message("Steve"), "Welcome to the server, Steve!");
    }
}
