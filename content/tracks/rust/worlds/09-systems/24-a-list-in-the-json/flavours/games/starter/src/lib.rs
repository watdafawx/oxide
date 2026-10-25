//! JSON values come in different shapes. Match on the shape.

use serde_json::Value;

/// The names in `players.sample` of a status JSON, in order.
/// No `sample` field (or `null`) is an empty list; a `sample` that is not a
/// list is `Err("players.sample is not a list")`. Entries without a text
/// `name` are skipped. Text that isn't JSON is an `Err` with its message.
pub fn who_is_online(json: &str) -> Result<Vec<String>, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    // TODO: match on the shape of v["players"]["sample"]: a list gives the
    //       text "name" of each entry; Null gives an empty list; any other
    //       shape is the error above
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_players_in_the_sample() {
        let json = r#"{"players": {"max": 20, "online": 2, "sample": [
            {"name": "Alex", "id": "a1"},
            {"name": "Sam", "id": "b2"}]}}"#;
        assert_eq!(who_is_online(json), Ok(vec!["Alex".to_string(), "Sam".to_string()]));
        let odd = r#"{"players": {"sample": [{"name": "Kim"}, {"id": "no-name"}, {"name": 7}]}}"#;
        assert_eq!(who_is_online(odd), Ok(vec!["Kim".to_string()]), "entries without a text name are skipped");
    }

    #[test]
    fn hidden_sample_is_empty_and_wrong_shape_errors() {
        assert_eq!(who_is_online(r#"{"players": {"online": 0}}"#), Ok(vec![]), "no sample at all");
        assert_eq!(who_is_online(r#"{"players": {"sample": null}}"#), Ok(vec![]), "sample is null");
        assert_eq!(
            who_is_online(r#"{"players": {"sample": "Alex"}}"#),
            Err("players.sample is not a list".to_string())
        );
        assert!(who_is_online("{").is_err(), "broken JSON");
    }
}
