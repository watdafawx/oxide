pub mod manifest;
pub mod resolver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_splits_name_and_score() {
        assert_eq!(manifest::parse_mod_line("sodium@5"), Some(("sodium".to_string(), 5)));
    }

    #[test]
    fn parse_line_rejects_garbage() {
        assert_eq!(manifest::parse_mod_line("no at sign here"), None);
        assert_eq!(manifest::parse_mod_line("sodium@latest"), None);
    }

    #[test]
    fn best_player_picks_the_highest_score() {
        let lines = ["jei@10", "sodium@30", "oops", "lithium@20"];
        assert_eq!(resolver::newest_mod(&lines), Some("sodium".to_string()));
        assert_eq!(resolver::newest_mod(&["junk"]), None);
    }
}
