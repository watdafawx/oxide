use serde::{Deserialize, Serialize};

// TODO: let serde write this struct as JSON and read it back (two more derives)
#[derive(Debug, Clone, PartialEq)]
pub struct ModpackProfile {
    pub name: String,
    pub mod_count: u32,
    pub favourites: Vec<String>,
    /// "YYYY-MM-DD", or None if the pack was never played.
    pub last_played: Option<String>,
}

/// The profile as JSON text.
pub fn to_json(profile: &ModpackProfile) -> serde_json::Result<String> {
    // TODO: turn the profile into JSON text with serde_json
    todo!()
}

/// A profile read back from JSON text.
pub fn from_json(json: &str) -> serde_json::Result<ModpackProfile> {
    // TODO: read the text into a ModpackProfile with serde_json
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ModpackProfile {
        ModpackProfile {
            name: "All the Mods".into(),
            mod_count: 340,
            favourites: vec!["sodium".into()],
            last_played: Some("2026-10-18".into()),
        }
    }

    #[test]
    fn to_json_writes_every_field() {
        let v: serde_json::Value = serde_json::from_str(&to_json(&sample()).unwrap()).unwrap();
        assert_eq!(v["name"], "All the Mods");
        assert_eq!(v["mod_count"], 340);
        assert_eq!(v["favourites"][0], "sodium");
        assert_eq!(v["last_played"], "2026-10-18");

        let mut fresh = sample();
        fresh.last_played = None;
        let v: serde_json::Value = serde_json::from_str(&to_json(&fresh).unwrap()).unwrap();
        assert!(v["last_played"].is_null(), "None must become null");
    }

    #[test]
    fn from_json_reads_it_back() {
        let text = r#"{"name": "Vanilla+", "mod_count": 12, "favourites": [], "last_played": null}"#;
        let p = from_json(text).unwrap();
        assert_eq!(p.mod_count, 12);
        assert_eq!(p.last_played, None);

        let back = from_json(&to_json(&sample()).unwrap()).unwrap();
        assert_eq!(back, sample(), "to_json then from_json gives the same profile");
    }
}
