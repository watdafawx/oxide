/// Adds `name` to the online list and reports who is first.
/// ["steve"] + "alex" -> "first online: steve, now 2 players"
pub fn join_and_report(online: &mut Vec<String>, name: &str) -> String {
    online.push(name.to_string());
    let first = &online[0];
    format!("first online: {first}, now {} players", online.len())
}

/// Kicks the last player in `online`: they move to the end of `kicked`. Does nothing if nobody is online.
pub fn kick_last(online: &mut Vec<String>, kicked: &mut Vec<String>) {
    if let Some(player) = online.pop() {
        kicked.push(player);
    }
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
