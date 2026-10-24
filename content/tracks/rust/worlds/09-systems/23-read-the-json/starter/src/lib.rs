//! The status answer is JSON text. Read one number out of it.

use serde_json::Value;

/// How many players are online, from a status JSON like
/// `{"players": {"online": 3, "max": 20}}`.
/// Text that isn't JSON is an `Err` with serde_json's message; a missing or
/// non-number `players.online` is `Err("missing players.online")`.
pub fn players_online(json: &str) -> Result<u64, String> {
    // TODO: parse the text into a Value (turn a parse error into its message
    //       text); then read players.online as a u64, and turn "not there"
    //       into the error "missing players.online"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_online_players() {
        let json = r#"{"version": {"name": "1.21.1", "protocol": 767},
                       "players": {"max": 20, "online": 3}}"#;
        assert_eq!(players_online(json), Ok(3));
        assert_eq!(players_online(r#"{"players": {"online": 0, "max": 5}}"#), Ok(0));
    }

    #[test]
    fn missing_or_broken_is_an_error() {
        assert!(players_online("not json").is_err(), "text that is not JSON");
        let missing = Err("missing players.online".to_string());
        assert_eq!(players_online("{}"), missing, "no players at all");
        assert_eq!(players_online(r#"{"players": {"max": 20}}"#), missing, "no online count");
        assert_eq!(players_online(r#"{"players": {"online": "three"}}"#), missing, "text, not a number");
    }
}
