use std::num::ParseIntError;

/// Newest manifest version this launcher understands.
pub const NEWEST: u32 = 20;

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    BadVersion(ParseIntError),
    TooNew(u32),
}

impl From<ParseIntError> for ManifestError {
    fn from(e: ParseIntError) -> Self {
        // TODO: wrap the parse error `e` in the ManifestError variant for bad versions
        todo!()
    }
}

/// Read a mod version. The `?` uses the From impl above. Given: you don't need to change it.
pub fn parse_version(text: &str) -> Result<u32, ManifestError> {
    let v: u32 = text.trim().parse()?;
    if v > NEWEST {
        return Err(ManifestError::TooNew(v));
    }
    Ok(v)
}

/// Add up every mod's version. The first broken version fails the whole fingerprint.
pub fn pack_fingerprint(versions: &[&str]) -> Result<u32, ManifestError> {
    // TODO: add up parse_version of every version, letting `?` hand back the first error
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_wraps_into_bad_version() {
        let parse_err = "new".parse::<u32>().unwrap_err();
        assert_eq!(ManifestError::from(parse_err.clone()), ManifestError::BadVersion(parse_err));
    }

    #[test]
    fn pack_fingerprint_stops_at_the_first_error() {
        assert_eq!(pack_fingerprint(&["1", "2", " 3 "]), Ok(6));
        assert_eq!(pack_fingerprint(&["1", "99", "new"]), Err(ManifestError::TooNew(99)));
        let parse_err = "new".parse::<u32>().unwrap_err();
        assert_eq!(pack_fingerprint(&["1", "new"]), Err(ManifestError::BadVersion(parse_err)));
    }
}
