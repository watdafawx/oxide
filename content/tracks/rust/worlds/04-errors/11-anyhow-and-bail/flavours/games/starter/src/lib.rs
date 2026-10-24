use anyhow::{Result, bail};

/// How much memory the game may use, in percent. parse_ram_percent(" 50 ") -> Ok(50)
pub fn parse_ram_percent(text: &str) -> Result<u32> {
    // TODO: parse the trimmed text as a u32 with `?`; above 100, bail! with "<n>% is more than all the memory"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_percent_parses_and_bad_text_fails() {
        assert_eq!(parse_ram_percent(" 50 ").unwrap(), 50);
        assert_eq!(parse_ram_percent("100").unwrap(), 100, "all of it is still allowed");
        let err = parse_ram_percent("lots").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string", "the ParseIntError, converted by ?");
    }

    #[test]
    fn too_much_ram_bails() {
        let err = parse_ram_percent("150").unwrap_err();
        assert_eq!(err.to_string(), "150% is more than all the memory");
    }
}
