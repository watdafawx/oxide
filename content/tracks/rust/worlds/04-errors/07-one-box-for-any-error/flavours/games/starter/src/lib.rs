use std::error::Error;

/// Read a pack header like "3,4.5": pack format 3, a 4.5 GB download.
pub fn parse_pack_header(text: &str) -> Result<(u32, f64), Box<dyn Error>> {
    // TODO: cut the text at the comma (no comma -> the error "missing comma"),
    //       then read the left half as a whole-number format and the right half as the size in GB, trimming each
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_format_and_size() {
        assert_eq!(parse_pack_header("3,4.5").unwrap(), (3, 4.5));
        assert_eq!(parse_pack_header(" 2 , 0.75 \n").unwrap(), (2, 0.75), "trim both halves");
    }

    #[test]
    fn missing_comma_says_so() {
        let err = parse_pack_header("3 4.5").unwrap_err();
        assert_eq!(err.to_string(), "missing comma");
    }

    #[test]
    fn bad_numbers_fit_in_the_box() {
        let err = parse_pack_header("new,4.5").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string", "a ParseIntError in the box");
        let err = parse_pack_header("3,huge").unwrap_err();
        assert_eq!(err.to_string(), "invalid float literal", "a ParseFloatError in the box");
    }
}
