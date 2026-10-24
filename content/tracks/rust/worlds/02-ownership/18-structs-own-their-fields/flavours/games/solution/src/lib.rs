/// A player record the server keeps while they are online.
pub struct Player {
    pub name: String,
    pub level: u32,
}

impl Player {
    pub fn new(name: &str, start_level: u32) -> Player {
        Player { name: name.to_string(), level: start_level }
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
