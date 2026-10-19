use anyhow::{Context, Result, bail};

/// Parse a mod version number. A failure should say "version must be a number".
pub fn parse_version(s: &str) -> Result<u32> {
    s.trim().parse::<u32>().context("version must be a number")
}

/// How much of the machine's RAM the pack may use. Accept 0 to 100 inclusive,
/// otherwise fail with "ram percent out of range: <n>".
pub fn check_ram_percent(n: i32) -> Result<i32> {
    if !(0..=100).contains(&n) {
        bail!("ram percent out of range: {n}");
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_port_parses() {
        assert_eq!(parse_version(" 12 ").unwrap(), 12);
    }

    #[test]
    fn bad_port_has_context_message() {
        let err = parse_version("latest").unwrap_err();
        assert_eq!(err.to_string(), "version must be a number");
    }

    #[test]
    fn context_keeps_the_original_cause() {
        let err = parse_version("latest").unwrap_err();
        assert_eq!(err.root_cause().to_string(), "invalid digit found in string");
    }

    #[test]
    fn check_percent_bails_when_out_of_range() {
        assert_eq!(check_ram_percent(50).unwrap(), 50);
        assert_eq!(check_ram_percent(0).unwrap(), 0);
        assert_eq!(check_ram_percent(100).unwrap(), 100);
        assert_eq!(check_ram_percent(150).unwrap_err().to_string(), "ram percent out of range: 150");
        assert_eq!(check_ram_percent(-1).unwrap_err().to_string(), "ram percent out of range: -1");
    }
}
