use std::error::Error;
use std::path::Path;

/// Parse a pack header `format,size_gb` such as "3,4.5".
pub fn parse_pack_header(text: &str) -> Result<(u32, f64), Box<dyn Error>> {
    let (left, right) = text.split_once(',').ok_or("missing comma")?;
    let format = left.trim().parse::<u32>()?;
    let size_gb = right.trim().parse::<f64>()?;
    Ok((format, size_gb))
}

/// Read a manifest file and parse its header. Missing file or bad header are both errors.
pub fn load_pack_header(path: &Path) -> Result<(u32, f64), Box<dyn Error>> {
    let text = std::fs::read_to_string(path)?;
    parse_pack_header(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_pair() {
        assert_eq!(parse_pack_header("3,4.5").unwrap(), (3, 4.5));
        assert_eq!(parse_pack_header(" 10 , 0.25 ").unwrap(), (10, 0.25));
    }

    #[test]
    fn missing_comma_is_an_error_with_message() {
        let err = parse_pack_header("34").unwrap_err();
        assert_eq!(err.to_string(), "missing comma");
    }

    #[test]
    fn bad_numbers_convert_into_the_box() {
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
