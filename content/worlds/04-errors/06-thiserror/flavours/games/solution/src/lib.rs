use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("mod not in manifest: {0}")]
    MissingMod(String),

    #[error("version is not a number: {0}")]
    BadVersion(#[from] ParseIntError),
}

/// Find `mod_name` in the manifest `pairs` (name, version) and parse its version number.
pub fn find_version(pairs: &[(&str, &str)], mod_name: &str) -> Result<u32, ManifestError> {
    let (_, version) = pairs
        .iter()
        .find(|(name, _)| *name == mod_name)
        .ok_or_else(|| ManifestError::MissingMod(mod_name.to_string()))?;
    Ok(version.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_mod_names_the_mod() {
        let err = find_version(&[("sodium", "5")], "lithium").unwrap_err();
        assert_eq!(err.to_string(), "mod not in manifest: lithium");
    }

    #[test]
    fn bad_version_converts_with_from() {
        let err = find_version(&[("sodium", "5x")], "sodium").unwrap_err();
        assert!(matches!(err, ManifestError::BadVersion(_)), "expected BadVersion, got {err:?}");
        assert_eq!(err.to_string(), "version is not a number: invalid digit found in string");
    }

    #[test]
    fn find_version_returns_the_number() {
        let pairs = [("sodium", "5"), ("lithium", "12")];
        assert_eq!(find_version(&pairs, "lithium").unwrap(), 12);
    }
}
