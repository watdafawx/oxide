use std::num::ParseIntError;

/// Newest build number this launcher understands.
const MAX_BUILD: u32 = 100;

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    BadVersion(ParseIntError),
    TooNew(u32),
}

impl From<ParseIntError> for ManifestError {
    fn from(e: ParseIntError) -> Self {
        ManifestError::BadVersion(e)
    }
}

/// Parse a build number. Builds above 100 are `TooNew`.
pub fn parse_build(s: &str) -> Result<u32, ManifestError> {
    let n: u32 = s.trim().parse()?;
    if n > MAX_BUILD { Err(ManifestError::TooNew(n)) } else { Ok(n) }
}

/// Add up the build numbers of every mod. Stop at the first error.
pub fn pack_fingerprint(builds: &[&str]) -> Result<u32, ManifestError> {
    let mut total = 0;
    for build in builds {
        total += parse_build(build)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_wraps_the_parse_error() {
        let parse_err = "x".parse::<u32>().unwrap_err();
        assert_eq!(ManifestError::from(parse_err.clone()), ManifestError::BadVersion(parse_err));
    }

    #[test]
    fn sum_all_stops_at_first_bad_input() {
        assert_eq!(pack_fingerprint(&["1", "2", "3"]), Ok(6));
        assert_eq!(pack_fingerprint(&["1", "500", "x"]), Err(ManifestError::TooNew(500)));
        let parse_err = "x".parse::<u32>().unwrap_err();
        assert_eq!(pack_fingerprint(&["1", "x"]), Err(ManifestError::BadVersion(parse_err)));
    }
}
