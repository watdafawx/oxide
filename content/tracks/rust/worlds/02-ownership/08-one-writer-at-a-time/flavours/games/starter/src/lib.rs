/// Adds `name` to the online list and reports who is first.
/// ["steve"] + "alex" -> "first online: steve, now 2 players"
pub fn join_and_report(online: &mut Vec<String>, name: &str) -> String {
    let first = &online[0];
    // TODO: this does not build: `first` still reads the list while `push` changes it.
    // Borrow the first player after the push instead.
    online.push(name.to_string());
    format!("first online: {first}, now {} players", online.len())
}

/// Kicks the last player in `online`: they move to the end of `kicked`. Does nothing if nobody is online.
pub fn kick_last(online: &mut Vec<String>, kicked: &mut Vec<String>) {
    // TODO: take the last player off `online`; if there was one, add them to `kicked`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_and_report_names_the_first_player() {
        let mut online = vec!["steve".to_string()];
        assert_eq!(join_and_report(&mut online, "alex"), "first online: steve, now 2 players");
        assert_eq!(online, vec!["steve", "alex"]);
    }

    #[test]
    fn kick_last_moves_player_to_kicked_list() {
        let mut online = vec!["steve".to_string(), "alex".to_string(), "bob".to_string()];
        let mut kicked = vec!["griefer".to_string()];
        kick_last(&mut online, &mut kicked);
        assert_eq!(online, vec!["steve", "alex"]);
        assert_eq!(kicked, vec!["griefer", "bob"]);
        let mut nobody: Vec<String> = Vec::new();
        kick_last(&mut nobody, &mut kicked);
        assert_eq!(kicked, vec!["griefer", "bob"], "nobody to kick from an empty server");
    }
}
