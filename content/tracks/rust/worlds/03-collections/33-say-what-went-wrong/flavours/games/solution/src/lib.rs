/// "64" -> Ok(64). Anything else -> Err("bad count '<text>'").
pub fn parse_count(text: &str) -> Result<u32, String> {
    text.trim().parse::<u32>().map_err(|_| format!("bad count '{text}'"))
}

/// ("64", "3") -> Ok((64, 3)). The error says which argument was bad:
/// "bad count 'x'" or "bad slot 'y'".
pub fn parse_give(count: &str, slot: &str) -> Result<(u32, u32), String> {
    let count_n = count.trim().parse::<u32>().map_err(|_| format!("bad count '{count}'"))?;
    let slot_n = slot.trim().parse::<u32>().map_err(|_| format!("bad slot '{slot}'"))?;
    Ok((count_n, slot_n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_count_names_the_bad_text() {
        assert_eq!(parse_count(" 64 "), Ok(64));
        assert_eq!(parse_count("x"), Err("bad count 'x'".to_string()));
    }

    #[test]
    fn parse_give_says_which_argument_failed() {
        assert_eq!(parse_give("64", "3"), Ok((64, 3)));
        assert_eq!(parse_give("lots", "3"), Err("bad count 'lots'".to_string()));
        assert_eq!(parse_give("64", "nine"), Err("bad slot 'nine'".to_string()));
    }
}
