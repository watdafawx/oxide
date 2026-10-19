use std::num::ParseIntError;

/// Newest build number this launcher understands.
const MAX_BUILD: u32 = 100;

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    BadVersion(ParseIntError),
    TooNew(u32),
}

impl From<ParseIntError> for ManifestError {
    fn from(_e: ParseIntError) -> Self {
        // TODO: wrap the error: ManifestError::BadVersion(e) (rename `_e` to `e`)
        todo!()
    }
}

/// Parse a build number. Builds above 100 are `TooNew`.
pub fn parse_build(s: &str) -> Result<u32, ManifestError> {
    let n: u32 = s.trim().parse()?;
    if n > MAX_BUILD { Err(ManifestError::TooNew(n)) } else { Ok(n) }
}

/// Add up the build numbers of every mod. Stop at the first error.
pub fn pack_fingerprint(builds: &[&str]) -> Result<u32, ManifestError> {
    // TODO: loop over builds, add parse_build(build)? to a running total
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_wraps_into_bad_version() {
        let parse_err = "x".parse::<u32>().unwrap_err();
        assert_eq!(ManifestError::from(parse_err.clone()), ManifestError::BadVersion(parse_err));
    }

    #[test]
    fn pack_fingerprint_stops_at_first_error() {
        assert_eq!(pack_fingerprint(&["1", "2", "3"]), Ok(6));
        assert_eq!(pack_fingerprint(&["1", "500", "x"]), Err(ManifestError::TooNew(500)));
        let parse_err = "x".parse::<u32>().unwrap_err();
        assert_eq!(pack_fingerprint(&["1", "x"]), Err(ManifestError::BadVersion(parse_err)));
    }
}
