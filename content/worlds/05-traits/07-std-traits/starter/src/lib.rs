use std::fmt;

// TODO: add #[derive(Debug, Clone, PartialEq, Default)] on the line above `pub struct Player`
pub struct Player {
    pub name: String,
    pub hp: u32,
}

/// `{:?}` text, e.g. `Player { name: "Ann", hp: 30 }`.
pub fn debug_text(p: &Player) -> String {
    // TODO: format p with {:?} (needs Debug)
    todo!()
}

/// Are the two players equal?
pub fn is_same(a: &Player, b: &Player) -> bool {
    // TODO: compare with == (needs PartialEq)
    todo!()
}

/// A copy of `p` with its hp replaced.
pub fn copy_with_hp(p: &Player, hp: u32) -> Player {
    // TODO: clone p (needs Clone), then set the clone's hp
    todo!()
}

/// A player with an empty name and 0 hp.
pub fn fresh_player() -> Player {
    // TODO: return Player::default() (needs Default)
    todo!()
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: write "<name> (<hp> hp)", e.g. "Ann (30 hp)"
        write!(f, "?")
    }
}

impl From<&str> for Player {
    fn from(name: &str) -> Self {
        // TODO: build a Player with this name and 100 hp
        Player { name: String::new(), hp: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ann() -> Player {
        Player { name: "Ann".to_string(), hp: 30 }
    }

    #[test]
    fn debug_and_equality_derive() {
        assert_eq!(debug_text(&ann()), "Player { name: \"Ann\", hp: 30 }");
        assert!(is_same(&ann(), &ann()), "same name and hp are equal");
        let mut other = ann();
        other.hp = 31;
        assert!(!is_same(&ann(), &other), "different hp is not equal");
    }

    #[test]
    fn clone_and_default_derive() {
        let hurt = copy_with_hp(&ann(), 5);
        assert_eq!(hurt.name, "Ann");
        assert_eq!(hurt.hp, 5);
        let fresh = fresh_player();
        assert_eq!(fresh.name, "");
        assert_eq!(fresh.hp, 0);
    }

    #[test]
    fn display_formats_player() {
        assert_eq!(ann().to_string(), "Ann (30 hp)");
        assert_eq!(format!("{}", Player { name: "Bo".to_string(), hp: 1 }), "Bo (1 hp)");
    }

    #[test]
    fn from_str_builds_player() {
        let p = Player::from("Ann");
        assert_eq!(p.name, "Ann");
        assert_eq!(p.hp, 100);
        let q: Player = "Bob".into();
        assert_eq!(q.name, "Bob", ".into() comes free with From");
    }
}
