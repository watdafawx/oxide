use serde::{Deserialize, Serialize};

/// The state of one objective after a check run.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
// TODO: spell every variant as a lowercase word in the JSON ("passed", "notrun")
pub enum Status {
    Passed,
    Failed,
    NotRun,
}

/// One card on the quest map, as the screen expects it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestCard {
    pub title: String,
    // TODO: the screen calls this key "type" (a Rust keyword), so rename the key only
    pub kind: String,
}

pub fn status_to_json(status: Status) -> serde_json::Result<String> {
    serde_json::to_string(&status)
}

pub fn card_to_json(card: &QuestCard) -> serde_json::Result<String> {
    serde_json::to_string(card)
}

pub fn card_from_json(json: &str) -> serde_json::Result<QuestCard> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statuses_are_lowercase_words() {
        assert_eq!(status_to_json(Status::NotRun).unwrap(), "\"notrun\"");
        assert_eq!(status_to_json(Status::Passed).unwrap(), "\"passed\"");
        let back: Status = serde_json::from_str("\"failed\"").unwrap();
        assert_eq!(back, Status::Failed);
    }

    #[test]
    fn the_kind_field_is_called_type() {
        let card = QuestCard {
            title: "Boss: Oxide in Rust".into(),
            kind: "boss".into(),
        };
        let v: serde_json::Value = serde_json::from_str(&card_to_json(&card).unwrap()).unwrap();
        assert_eq!(v["type"], "boss", "the key must be \"type\"");
        assert!(v.get("kind").is_none(), "there must be no \"kind\" key");

        let read = card_from_json(r#"{"title": "Loops", "type": "exercise"}"#).unwrap();
        assert_eq!(read.kind, "exercise");
    }
}
