pub mod parser;
pub mod report;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_splits_name_and_score() {
        assert_eq!(parser::parse_line("ferris: 42"), Some(("ferris".to_string(), 42)));
    }

    #[test]
    fn parse_line_rejects_garbage() {
        assert_eq!(parser::parse_line("no colon here"), None);
        assert_eq!(parser::parse_line("ferris: lots"), None);
    }

    #[test]
    fn best_player_picks_the_highest_score() {
        let lines = ["ana: 10", "bob: 30", "oops", "cy: 20"];
        assert_eq!(report::best_player(&lines), Some("bob".to_string()));
        assert_eq!(report::best_player(&["junk"]), None);
    }
}
