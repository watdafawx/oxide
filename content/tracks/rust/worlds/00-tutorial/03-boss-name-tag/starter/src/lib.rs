/// `ferris` -> `FERRIS`
pub fn shout(name: &str) -> String {
    // TODO: uppercase the name
    todo!()
}

/// `("ferris", 3)` -> `[FERRIS] Lv 3`
pub fn name_tag(name: &str, level: u32) -> String {
    // TODO: use shout() and format!
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouts_the_name() {
        assert_eq!(shout("ferris"), "FERRIS");
        assert_eq!(shout("Oxide"), "OXIDE");
    }

    #[test]
    fn tag_shows_name_and_level() {
        assert_eq!(name_tag("ferris", 3), "[FERRIS] Lv 3");
        assert_eq!(name_tag("mia", 12), "[MIA] Lv 12");
    }
}
