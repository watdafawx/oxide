use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherSettings {
    pub loader: String,
    pub auto_update: bool,
    pub keep_versions: u32,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        LauncherSettings {
            loader: "fabric".into(),
            auto_update: true,
            keep_versions: 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ModpackProfile {
    pub name: String,
    pub mod_count: u32,
    pub favourites: Vec<String>,
    pub last_played: Option<String>,
    pub settings: LauncherSettings,
}

/// A profile read from JSON text (from last quest).
pub fn from_json(json: &str) -> serde_json::Result<ModpackProfile> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_old_save_with_only_mod_count_still_loads() {
        let p = from_json(r#"{"mod_count": 120}"#).expect("an old save must still load");
        assert_eq!(p.mod_count, 120);
        assert!(p.favourites.is_empty());
        assert_eq!(p.last_played, None);
        assert_eq!(p.settings, LauncherSettings::default());
    }

    #[test]
    fn missing_settings_use_their_defaults() {
        let p = from_json(r#"{"mod_count": 5, "settings": {"auto_update": false}}"#)
            .expect("settings with only some fields must still load");
        assert!(!p.settings.auto_update, "a field in the file wins over the default");
        assert_eq!(p.settings.loader, "fabric");
        assert_eq!(p.settings.keep_versions, 3);
    }
}
