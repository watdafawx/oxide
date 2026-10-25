use serde::{Deserialize, Serialize};

/// Where a mod stands in the pack.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModStatus {
    Enabled,
    Disabled,
    NotInstalled,
}

/// One downloadable file, as the mod site describes it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModFile {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
}

pub fn status_to_json(status: ModStatus) -> serde_json::Result<String> {
    serde_json::to_string(&status)
}

pub fn file_to_json(file: &ModFile) -> serde_json::Result<String> {
    serde_json::to_string(file)
}

pub fn file_from_json(json: &str) -> serde_json::Result<ModFile> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statuses_are_lowercase_words() {
        assert_eq!(status_to_json(ModStatus::NotInstalled).unwrap(), "\"notinstalled\"");
        assert_eq!(status_to_json(ModStatus::Enabled).unwrap(), "\"enabled\"");
        let back: ModStatus = serde_json::from_str("\"disabled\"").unwrap();
        assert_eq!(back, ModStatus::Disabled);
    }

    #[test]
    fn the_kind_field_is_called_type() {
        let file = ModFile {
            name: "Faithful 32x".into(),
            kind: "resourcepack".into(),
        };
        let v: serde_json::Value = serde_json::from_str(&file_to_json(&file).unwrap()).unwrap();
        assert_eq!(v["type"], "resourcepack", "the key must be \"type\"");
        assert!(v.get("kind").is_none(), "there must be no \"kind\" key");

        let read = file_from_json(r#"{"name": "sodium", "type": "mod"}"#).unwrap();
        assert_eq!(read.kind, "mod");
    }
}
