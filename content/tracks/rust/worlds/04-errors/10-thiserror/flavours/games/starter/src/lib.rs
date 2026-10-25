use std::num::ParseIntError;
use thiserror::Error;

/// Newest manifest version this launcher understands.
pub const NEWEST: u32 = 20;

#[derive(Debug, PartialEq, Error)]
pub enum ManifestError {
    // TODO: the message "no version"
    #[error("TODO")]
    MissingVersion,
    // TODO: the message "version is not a number: <the parse error>", and let `?` convert into it
    #[error("TODO")]
    BadVersion(ParseIntError),
    #[error("version {0} is too new")]
    TooNew(u32),
}

/// Read a mod version, up to NEWEST.
pub fn parse_version(text: &str) -> Result<u32, ManifestError> {
    // TODO: trim; empty -> MissingVersion; parse with `?` (the #[from] does the converting); above NEWEST -> TooNew
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_come_from_the_attributes() {
        assert_eq!(ManifestError::MissingVersion.to_string(), "no version");
        let parse_err = "new".parse::<u32>().unwrap_err();
        assert_eq!(
            ManifestError::BadVersion(parse_err).to_string(),
            "version is not a number: invalid digit found in string"
        );
        assert_eq!(ManifestError::TooNew(99).to_string(), "version 99 is too new");
    }

    #[test]
    fn question_mark_converts_with_from() {
        assert_eq!(parse_version(" 7 "), Ok(7));
        assert_eq!(parse_version(""), Err(ManifestError::MissingVersion));
        assert_eq!(parse_version("99"), Err(ManifestError::TooNew(99)));
        let parse_err = "new".parse::<u32>().unwrap_err();
        assert_eq!(parse_version("new"), Err(ManifestError::BadVersion(parse_err)));
    }
}
