use std::fmt;

/// Newest manifest version this launcher understands.
pub const NEWEST: u32 = 20;

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    MissingVersion,
    BadVersion(String),
    TooNew(u32),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ManifestError::MissingVersion => write!(f, "no version"),
            ManifestError::BadVersion(text) => write!(f, "'{text}' is not a number"),
            ManifestError::TooNew(v) => write!(f, "version {v} is too new"),
        }
    }
}

/// From the last quests. Given: you don't need to change it.
pub fn parse_version(text: &str) -> Result<u32, ManifestError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(ManifestError::MissingVersion);
    }
    let v = match text.parse::<u32>() {
        Ok(v) => v,
        Err(_) => return Err(ManifestError::BadVersion(text.to_string())),
    };
    if v > NEWEST {
        return Err(ManifestError::TooNew(v));
    }
    Ok(v)
}

/// explain("7") -> "version 7", explain("new") -> "skipped: 'new' is not a number"
pub fn explain(text: &str) -> String {
    match parse_version(text) {
        Ok(v) => format!("version {v}"),
        Err(e) => format!("skipped: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_manifest_error_prints_a_message() {
        assert_eq!(ManifestError::MissingVersion.to_string(), "no version");
        assert_eq!(ManifestError::BadVersion("new".to_string()).to_string(), "'new' is not a number");
        assert_eq!(ManifestError::TooNew(99).to_string(), "version 99 is too new");
    }

    #[test]
    fn explain_puts_the_error_in_a_sentence() {
        assert_eq!(explain("7"), "version 7");
        assert_eq!(explain("new"), "skipped: 'new' is not a number");
        assert_eq!(explain(""), "skipped: no version");
    }
}
