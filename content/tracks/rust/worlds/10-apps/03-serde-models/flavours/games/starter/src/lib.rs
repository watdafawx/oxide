use serde::{Deserialize, Serialize};

// TODO: also derive Serialize and Deserialize
#[derive(Debug, Clone, PartialEq)]
pub struct LauncherSettings {
    pub loader: String,
    pub auto_update: bool,
    pub keep_versions: u32,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        // TODO: loader "fabric", auto_update true, keep_versions 3
        LauncherSettings {
            loader: String::new(),
            auto_update: false,
            keep_versions: 0,
        }
    }
}

// TODO: also derive Serialize and Deserialize, and tell serde to fill missing fields from Default
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModpackProfile {
    pub name: String,
    pub mc_version: String,
    pub mod_count: u32,
    pub update_streak: u32,
    /// "YYYY-MM-DD" or null.
    pub last_played: Option<String>,
    pub favourites: Vec<String>,
    pub disabled: Vec<String>,
    pub settings: LauncherSettings,
}

// TODO: also derive Serialize and Deserialize, and tell serde to write the variant names in lowercase
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModStatus {
    Enabled,
    Disabled,
    NotInstalled,
}

pub fn to_json(profile: &ModpackProfile) -> serde_json::Result<String> {
    // TODO: turn the profile into a JSON string with serde_json
    todo!()
}

pub fn from_json(json: &str) -> serde_json::Result<ModpackProfile> {
    // TODO: parse the JSON text back into a profile with serde_json
    todo!()
}

pub fn status_to_json(status: ModStatus) -> serde_json::Result<String> {
    // TODO: turn the status into a JSON string with serde_json
    let _ = status;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ModpackProfile {
        ModpackProfile {
            name: "All the Mods".into(),
            mc_version: "1.21.1".into(),
            mod_count: 340,
            update_streak: 4,
            last_played: Some("2026-10-18".into()),
            favourites: vec!["sodium".into()],
            disabled: vec![],
            settings: LauncherSettings::default(),
        }
    }

    #[test]
    fn profile_serializes_with_the_api_field_names() {
        let json = to_json(&sample()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["mod_count"], 340);
        assert_eq!(v["last_played"], "2026-10-18");
        assert_eq!(v["settings"]["keep_versions"], 3);
        assert_eq!(v["settings"]["auto_update"], true);

        let mut p = sample();
        p.last_played = None;
        let v: serde_json::Value = serde_json::from_str(&to_json(&p).unwrap()).unwrap();
        assert!(v["last_played"].is_null(), "None must become null");
    }

    #[test]
    fn profile_round_trips_through_json() {
        let p = sample();
        let back = from_json(&to_json(&p).unwrap()).unwrap();
        assert_eq!(back, p);
    }

    #[test]
    fn an_old_save_with_missing_fields_still_loads() {
        let p = from_json(r#"{"mod_count": 120}"#).unwrap();
        assert_eq!(p.mod_count, 120);
        assert_eq!(p.settings.keep_versions, 3);
        assert!(p.favourites.is_empty());
        assert_eq!(p.last_played, None);
    }

    #[test]
    fn mod_status_is_lowercase_in_json() {
        assert_eq!(status_to_json(ModStatus::NotInstalled).unwrap(), "\"notinstalled\"");
        assert_eq!(status_to_json(ModStatus::Enabled).unwrap(), "\"enabled\"");
    }
}
