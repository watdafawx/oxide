pub struct Player {
    pub name: String,
    pub hp: u32,
}

impl Player {
    /// A new player starts with 100 hp.
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            hp: 100,
        }
    }

    /// Lose `damage` hp, but never go below 0.
    pub fn take_hit(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }

    /// Alive while hp is above 0.
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_at_full_health() {
        let p = Player::new("mia");
        assert_eq!(p.hp, 100);
        assert_eq!(p.name, "mia");
    }

    #[test]
    fn take_hit_lowers_hp() {
        let mut p = Player::new("mia");
        p.take_hit(30);
        assert_eq!(p.hp, 70);
    }

    #[test]
    fn hp_never_goes_below_zero() {
        let mut p = Player::new("mia");
        p.take_hit(250);
        assert_eq!(p.hp, 0, "hp is a u32, so it must stop at 0");
    }

    #[test]
    fn is_alive_checks_hp() {
        let mut p = Player::new("mia");
        assert!(p.is_alive(), "a fresh player is alive");
        p.take_hit(100);
        assert!(!p.is_alive(), "0 hp means dead");
    }
}
