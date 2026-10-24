/// "Steve joined for the first time!" on a first join, "Steve joined the game" after that.
pub fn join_message(name: &str, first_join: bool) -> String {
    if first_join {
        format!("{name} joined for the first time!")
    } else {
        format!("{name} joined the game")
    }
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
