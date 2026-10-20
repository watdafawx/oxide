use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Loader {
    Fabric,
    Forge,
    NeoForge,
}

#[derive(Debug, PartialEq)]
pub struct ModEntry {
    pub name: String,
    pub version: u32,
    pub loader: Loader,
}

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    MissingMod,
    BadVersion(String),
    UnknownLoader(String),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: write one message per variant: "line has no mod name", "'<v>' is not a version number", "unknown loader: <l>"
        todo!()
    }
}

impl std::error::Error for ManifestError {}

/// Parse a manifest line such as `sodium@5 fabric`.
pub fn parse_mod_line(line: &str) -> Result<ModEntry, ManifestError> {
    // TODO: take the first word with `split_whitespace().next()` (none -> MissingMod). Cut it at '@' with `split_once`;
    //       an empty name -> MissingMod. A version that is not a number -> BadVersion carrying the version text.
    //       Then match the next word: fabric, forge, neoforge; anything else (or no word) -> UnknownLoader carrying it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_errors_display_clearly() {
        assert_eq!(ManifestError::MissingMod.to_string(), "line has no mod name");
        assert_eq!(
            ManifestError::BadVersion("old".to_string()).to_string(),
            "'old' is not a version number"
        );
        assert_eq!(
            ManifestError::UnknownLoader("quilt".to_string()).to_string(),
            "unknown loader: quilt"
        );
    }

    #[test]
    fn parse_mod_line_reports_each_failure() {
        assert_eq!(
            parse_mod_line(" sodium@5 fabric "),
            Ok(ModEntry { name: "sodium".to_string(), version: 5, loader: Loader::Fabric })
        );
        assert_eq!(parse_mod_line("   "), Err(ManifestError::MissingMod));
        assert_eq!(parse_mod_line("@5 fabric"), Err(ManifestError::MissingMod));
        assert_eq!(
            parse_mod_line("sodium@old fabric"),
            Err(ManifestError::BadVersion("old".to_string()))
        );
        assert_eq!(
            parse_mod_line("sodium@5 quilt"),
            Err(ManifestError::UnknownLoader("quilt".to_string()))
        );
    }

    #[test]
    fn manifest_error_works_as_dyn_error() {
        let err = parse_mod_line("").unwrap_err();
        let dynamic: &dyn std::error::Error = &err;
        assert_eq!(dynamic.to_string(), "line has no mod name");
    }
}
