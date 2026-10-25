//! The status answer is JSON text. Read one number out of it.

use serde_json::Value;

/// How many players are online, from a status JSON like
/// `{"players": {"online": 3, "max": 20}}`.
/// Text that isn't JSON is an `Err` with serde_json's message; a missing or
/// non-number `players.online` is `Err("missing players.online")`.
pub fn online_players(json: &str) -> Result<u64, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    v["players"]["online"]
        .as_u64()
        .ok_or("missing players.online".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_how_many_are_online() {
        let json = r#"{"version": {"name": "1.21.1", "protocol": 767},
                       "players": {"max": 20, "online": 3}}"#;
        assert_eq!(online_players(json), Ok(3));
        assert_eq!(online_players(r#"{"players": {"online": 0, "max": 5}}"#), Ok(0));
    }

    #[test]
    fn missing_or_broken_status_is_an_error() {
        assert!(online_players("not json").is_err(), "text that is not JSON");
        let missing = Err("missing players.online".to_string());
        assert_eq!(online_players("{}"), missing, "no players at all");
        assert_eq!(online_players(r#"{"players": {"max": 20}}"#), missing, "no online count");
        assert_eq!(online_players(r#"{"players": {"online": "three"}}"#), missing, "text, not a number");
    }
}
