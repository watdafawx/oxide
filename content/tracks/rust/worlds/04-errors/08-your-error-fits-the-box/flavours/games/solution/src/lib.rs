use std::error::Error;
use std::fmt;

/// Newest manifest version this launcher understands.
pub const NEWEST: u32 = 20;

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    TooNew(u32),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ManifestError::TooNew(v) => write!(f, "version {v} is too new"),
        }
    }
}

impl std::error::Error for ManifestError {}

/// Versions above NEWEST are too new for this launcher.
pub fn check_version(v: u32) -> Result<u32, ManifestError> {
    if v > NEWEST {
        return Err(ManifestError::TooNew(v));
    }
    Ok(v)
}

/// Read and check a version. Two kinds of error, one box. Given: you don't need to change it.
pub fn load_version(text: &str) -> Result<u32, Box<dyn Error>> {
    let v: u32 = text.trim().parse()?;
    let v = check_version(v)?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_good_version_loads() {
        assert_eq!(load_version(" 7 ").unwrap(), 7);
    }

    #[test]
    fn manifest_error_travels_in_the_box() {
        let err = load_version("99").unwrap_err();
        assert_eq!(err.to_string(), "version 99 is too new");
        assert_eq!(err.downcast_ref::<ManifestError>(), Some(&ManifestError::TooNew(99)), "the box still holds your ManifestError");
    }
}
