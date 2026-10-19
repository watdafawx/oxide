/// A player record that owns its name.
#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub level: u32,
}

/// A light view of a player that borrows the name from somewhere else.
#[derive(Debug, PartialEq)]
pub struct PlayerRef<'a> {
    pub name: &'a str,
    pub level: u32,
}

impl Player {
    pub fn new(name: &str, level: u32) -> Player {
        Player { name: name.to_string(), level }
    }

    /// A borrowed view of this player: same data, no copying.
    pub fn view(&self) -> PlayerRef<'_> {
        PlayerRef { name: &self.name, level: self.level }
    }
}

impl<'a> PlayerRef<'a> {
    pub fn new(name: &'a str, level: u32) -> PlayerRef<'a> {
        PlayerRef { name, level }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_outlives_its_source() {
        let player = {
            let typed = String::from("ferris");
            Player::new(&typed, 3)
        };
        assert_eq!(player.name, "ferris");
        assert_eq!(player.level, 3);
    }

    #[test]
    fn borrowed_reads_from_source() {
        let text = String::from("ana");
        let view = PlayerRef::new(&text, 7);
        assert_eq!(view.name, "ana");
        assert_eq!(view.level, 7);
        assert_eq!(view.name.as_ptr(), text.as_ptr(), "must point at the original text, not a copy");
    }

    #[test]
    fn as_borrowed_points_into_owned() {
        let player = Player::new("bo", 2);
        let view = player.view();
        assert_eq!(view, PlayerRef { name: "bo", level: 2 });
        assert_eq!(view.name.as_ptr(), player.name.as_ptr(), "must borrow, not copy");
    }
}
