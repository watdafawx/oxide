use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
    // TODO: change the message to "mod not in manifest: {0}"
    #[error("TODO")]
    MissingMod(String),

    // TODO: message "version is not a number: {0}" and put #[from] on the field
    #[error("TODO")]
    BadVersion(ParseIntError),
}

/// Find `mod_name` in the manifest `pairs` (name, version) and parse its version number.
pub fn find_version(pairs: &[(&str, &str)], mod_name: &str) -> Result<u32, ManifestError> {
    // TODO: find the pair whose name matches (ok_or_else -> ManifestError::MissingMod), then parse the version with `?` (needs #[from] on BadVersion)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_key_message_names_the_key() {
        let err = find_version(&[("sodium", "5")], "lithium").unwrap_err();
        assert_eq!(err.to_string(), "mod not in manifest: lithium");
    }

    #[test]
    fn bad_port_converts_with_from() {
        let err = find_version(&[("sodium", "5x")], "sodium").unwrap_err();
        assert!(matches!(err, ManifestError::BadVersion(_)), "expected BadVersion, got {err:?}");
        assert_eq!(err.to_string(), "version is not a number: invalid digit found in string");
    }

    #[test]
    fn read_port_finds_a_valid_port() {
        let pairs = [("sodium", "5"), ("lithium", "12")];
        assert_eq!(find_version(&pairs, "lithium").unwrap(), 12);
    }
}
