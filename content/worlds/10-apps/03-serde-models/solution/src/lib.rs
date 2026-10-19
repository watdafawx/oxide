use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub editor: String,
    pub sound: bool,
    pub focus_minutes: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            editor: "code".into(),
            sound: true,
            focus_minutes: 25,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    pub xp: u32,
    pub level: u32,
    pub xp_into_level: u32,
    pub xp_for_next: u32,
    pub streak_days: u32,
    pub best_streak: u32,
    /// "YYYY-MM-DD" or null.
    pub last_active: Option<String>,
    pub achievements: Vec<String>,
    pub completed: Vec<String>,
    pub started: Vec<String>,
    pub settings: Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectiveStatus {
    Passed,
    Failed,
    NotRun,
}

pub fn to_json(profile: &Profile) -> serde_json::Result<String> {
    serde_json::to_string(profile)
}

pub fn from_json(json: &str) -> serde_json::Result<Profile> {
    serde_json::from_str(json)
}

pub fn status_to_json(status: ObjectiveStatus) -> serde_json::Result<String> {
    serde_json::to_string(&status)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Profile {
        Profile {
            xp: 340,
            level: 3,
            xp_into_level: 40,
            xp_for_next: 200,
            streak_days: 4,
            best_streak: 9,
            last_active: Some("2026-10-18".into()),
            achievements: vec!["first-quest".into()],
            completed: vec!["01-foundations/01-variables".into()],
            started: vec![],
            settings: Settings::default(),
        }
    }

    #[test]
    fn profile_serializes_with_api_field_names() {
        let json = to_json(&sample()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["xp_into_level"], 40);
        assert_eq!(v["last_active"], "2026-10-18");
        assert_eq!(v["settings"]["focus_minutes"], 25);
        assert_eq!(v["settings"]["sound"], true);

        let mut p = sample();
        p.last_active = None;
        let v: serde_json::Value = serde_json::from_str(&to_json(&p).unwrap()).unwrap();
        assert!(v["last_active"].is_null(), "None must become null");
    }

    #[test]
    fn profile_round_trips() {
        let p = sample();
        let back = from_json(&to_json(&p).unwrap()).unwrap();
        assert_eq!(back, p);
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let p = from_json(r#"{"xp": 120}"#).unwrap();
        assert_eq!(p.xp, 120);
        assert_eq!(p.settings.focus_minutes, 25);
        assert!(p.completed.is_empty());
        assert_eq!(p.last_active, None);
    }

    #[test]
    fn status_is_lowercase_in_json() {
        assert_eq!(status_to_json(ObjectiveStatus::NotRun).unwrap(), "\"notrun\"");
        assert_eq!(status_to_json(ObjectiveStatus::Passed).unwrap(), "\"passed\"");
    }
}
