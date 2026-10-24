use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub sound: bool,
    pub focus_minutes: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sound: true,
            focus_minutes: 25,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    pub xp: u32,
    pub completed: Vec<String>,
    pub last_active: Option<String>,
    pub settings: Settings,
}

/// A profile read from JSON text (from last quest).
pub fn from_json(json: &str) -> serde_json::Result<Profile> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_old_save_with_only_xp_still_loads() {
        let p = from_json(r#"{"xp": 120}"#).expect("an old save must still load");
        assert_eq!(p.xp, 120);
        assert!(p.completed.is_empty());
        assert_eq!(p.last_active, None);
        assert_eq!(p.settings, Settings::default());
    }

    #[test]
    fn missing_settings_use_their_defaults() {
        let p = from_json(r#"{"xp": 5, "settings": {"sound": false}}"#)
            .expect("settings with only some fields must still load");
        assert!(!p.settings.sound, "a field in the file wins over the default");
        assert_eq!(p.settings.focus_minutes, 25);
    }
}
