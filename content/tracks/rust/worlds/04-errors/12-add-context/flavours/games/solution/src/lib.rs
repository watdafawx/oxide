use anyhow::{Context, Result};

/// A version field. A bad one says "version must be a number" on top of the parse error.
pub fn parse_version(text: &str) -> Result<u32> {
    text.trim().parse::<u32>().context("version must be a number")
}

/// The same, with the mod's name on top: "mod sodium".
pub fn version_of_mod(name: &str, text: &str) -> Result<u32> {
    parse_version(text).with_context(|| format!("mod {name}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_sits_on_top_of_the_root_cause() {
        assert_eq!(parse_version(" 7 ").unwrap(), 7);
        let err = parse_version("new").unwrap_err();
        assert_eq!(err.to_string(), "version must be a number");
        assert_eq!(err.root_cause().to_string(), "invalid digit found in string");
    }

    #[test]
    fn with_context_names_the_mod() {
        assert_eq!(version_of_mod("lithium", "4").unwrap(), 4);
        let err = version_of_mod("sodium", "new").unwrap_err();
        assert_eq!(err.to_string(), "mod sodium");
        assert_eq!(format!("{err:#}"), "mod sodium: version must be a number: invalid digit found in string");
    }
}
