/// A player record the server keeps while they are online.
pub struct Player {
    // TODO: this does not build: a field can't just borrow text. Give the player their own String.
    pub name: &str,
    pub level: u32,
}

impl Player {
    pub fn new(name: &str, start_level: u32) -> Player {
        // TODO: once `name` is a String, store an owned copy of the text here
        Player { name: name, level: start_level }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_outlives_the_typed_name() {
        let player = {
            let typed = String::from("ferris");
            Player::new(&typed, 3)
        };
        assert_eq!(player.name, "ferris");
        assert_eq!(player.level, 3);
    }
}
