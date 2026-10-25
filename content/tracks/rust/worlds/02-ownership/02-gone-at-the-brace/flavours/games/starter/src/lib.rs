/// "Steve joined for the first time!" on a first join, "Steve joined the game" after that.
pub fn join_message(name: &str, first_join: bool) -> String {
    if first_join {
        let message = format!("{name} joined for the first time!");
    } else {
        let message = format!("{name} joined the game");
    }
    // TODO: this does not build: each `message` is gone at the `}` of its branch.
    // Make the `if` itself give back the text.
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_join_is_announced() {
        assert_eq!(join_message("Steve", true), "Steve joined for the first time!");
    }

    #[test]
    fn rejoin_says_joined_the_game() {
        assert_eq!(join_message("Steve", false), "Steve joined the game");
    }
}
