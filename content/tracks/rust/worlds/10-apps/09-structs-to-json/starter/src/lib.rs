use serde::{Deserialize, Serialize};

// TODO: let serde write this struct as JSON and read it back (two more derives)
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub xp: u32,
    pub level: u32,
    pub completed: Vec<String>,
    /// "YYYY-MM-DD", or None before the first finished quest.
    pub last_active: Option<String>,
}

/// The profile as JSON text.
pub fn to_json(profile: &Profile) -> serde_json::Result<String> {
    // TODO: turn the profile into JSON text with serde_json
    todo!()
}

/// A profile read back from JSON text.
pub fn from_json(json: &str) -> serde_json::Result<Profile> {
    // TODO: read the text into a Profile with serde_json
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Profile {
        Profile {
            xp: 340,
            level: 4,
            completed: vec!["rust/01-foundations/01-a-name-for-a-number".into()],
            last_active: Some("2026-10-18".into()),
        }
    }

    #[test]
    fn to_json_writes_every_field() {
        let v: serde_json::Value = serde_json::from_str(&to_json(&sample()).unwrap()).unwrap();
        assert_eq!(v["xp"], 340);
        assert_eq!(v["level"], 4);
        assert_eq!(v["completed"][0], "rust/01-foundations/01-a-name-for-a-number");
        assert_eq!(v["last_active"], "2026-10-18");

        let mut new_player = sample();
        new_player.last_active = None;
        let v: serde_json::Value = serde_json::from_str(&to_json(&new_player).unwrap()).unwrap();
        assert!(v["last_active"].is_null(), "None must become null");
    }

    #[test]
    fn from_json_reads_it_back() {
        let text = r#"{"xp": 50, "level": 1, "completed": [], "last_active": null}"#;
        let p = from_json(text).unwrap();
        assert_eq!(p.xp, 50);
        assert_eq!(p.last_active, None);

        let back = from_json(&to_json(&sample()).unwrap()).unwrap();
        assert_eq!(back, sample(), "to_json then from_json gives the same profile");
    }
}
