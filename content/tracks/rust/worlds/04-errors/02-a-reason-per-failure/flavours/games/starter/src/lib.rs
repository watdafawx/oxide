/// Newest manifest version this launcher understands.
pub const NEWEST: u32 = 20;

/// Every way a typed mod version can be wrong.
/// (`#[derive(Debug, PartialEq)]` lets the tests compare and print errors. Two quests from now explain that line.)
#[derive(Debug, PartialEq)]
pub enum ManifestError {
    MissingVersion,
    BadVersion(String),
    TooNew(u32),
}

/// Read the version part of `sodium@7`. parse_version(" 7 ") -> Ok(7)
pub fn parse_version(text: &str) -> Result<u32, ManifestError> {
    // TODO: trim the text. Nothing left (`is_empty()`) -> MissingVersion; not a whole number -> BadVersion with the
    //       trimmed text; above NEWEST -> TooNew with the number; otherwise the version
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_good_version_is_ok() {
        assert_eq!(parse_version(" 7 "), Ok(7));
        assert_eq!(parse_version("20"), Ok(20), "the newest known version still loads");
    }

    #[test]
    fn each_failure_has_its_own_variant() {
        assert_eq!(parse_version("   "), Err(ManifestError::MissingVersion));
        assert_eq!(parse_version("new"), Err(ManifestError::BadVersion("new".to_string())));
        assert_eq!(parse_version("99"), Err(ManifestError::TooNew(99)));
    }
}
