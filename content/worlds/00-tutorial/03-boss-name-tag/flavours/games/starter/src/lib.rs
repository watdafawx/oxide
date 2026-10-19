/// `ferris` -> `FERRIS`
pub fn loud_name(name: &str) -> String {
    // TODO: uppercase the name
    todo!()
}

/// `("ferris", 3)` -> `[FERRIS] Lv 3`
pub fn player_tag(name: &str, level: u32) -> String {
    // TODO: use loud_name() and format!
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loud_name_is_uppercase() {
        assert_eq!(loud_name("ferris"), "FERRIS");
        assert_eq!(loud_name("Oxide"), "OXIDE");
    }

    #[test]
    fn player_tag_shows_name_and_level() {
        assert_eq!(player_tag("ferris", 3), "[FERRIS] Lv 3");
        assert_eq!(player_tag("mia", 12), "[MIA] Lv 12");
    }
}
