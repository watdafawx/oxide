use std::error::Error;
use std::path::Path;

/// Parse a pack header `format,size_gb` such as "3,4.5".
pub fn parse_pack_header(text: &str) -> Result<(u32, f64), Box<dyn Error>> {
    // TODO: split at the comma (no comma -> an error saying "missing comma"), trim both halves, parse the left as u32 and the right as f64, let `?` pass failures on
    todo!()
}

/// Read a manifest file and parse its header. Missing file or bad header are both errors.
pub fn load_pack_header(path: &Path) -> Result<(u32, f64), Box<dyn Error>> {
    // TODO: read the file into a String (an io error is fine, the Box takes any error), then parse its header
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pack_header_reads_a_pair() {
        assert_eq!(parse_pack_header("3,4.5").unwrap(), (3, 4.5));
        assert_eq!(parse_pack_header(" 10 , 0.25 ").unwrap(), (10, 0.25));
    }

    #[test]
    fn pack_header_without_comma_says_missing_comma() {
        let err = parse_pack_header("34").unwrap_err();
        assert_eq!(err.to_string(), "missing comma");
    }

    #[test]
    fn pack_header_errors_fit_in_the_box() {
        let int_err = parse_pack_header("x,1.0").unwrap_err();
        assert_eq!(int_err.to_string(), "invalid digit found in string");
        let float_err = parse_pack_header("1,y").unwrap_err();
        assert_eq!(float_err.to_string(), "invalid float literal");

        let path = std::env::temp_dir().join(format!("oxide_pack_header_{}.txt", std::process::id()));
        std::fs::write(&path, "2,1.5\n").unwrap();
        let loaded = load_pack_header(&path);
        let _ = std::fs::remove_file(&path);
        assert_eq!(loaded.unwrap(), (2, 1.5));

        let io_err = load_pack_header(&path).unwrap_err();
        assert!(io_err.downcast_ref::<std::io::Error>().is_some(), "a missing file is an io::Error");
    }
}
