/// "12" -> Ok(12). Anything else -> Err("bad qty '<text>'").
pub fn parse_qty(text: &str) -> Result<u32, String> {
    // TODO: parse the trimmed text as a u32; replace a parse error with "bad qty '<text>'"
    Ok(0)
}

/// ("3", "150") -> Ok((3, 150)). The error says which field was bad:
/// "bad qty 'x'" or "bad price 'y'".
pub fn parse_pair(qty: &str, price: &str) -> Result<(u32, u32), String> {
    // TODO: parse qty with map_err(...)? and the message "bad qty '<qty>'"
    // TODO: parse price the same way, with "bad price '<price>'", and give back Ok of the pair
    Ok((0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_qty_names_the_bad_text() {
        assert_eq!(parse_qty(" 12 "), Ok(12));
        assert_eq!(parse_qty("x"), Err("bad qty 'x'".to_string()));
    }

    #[test]
    fn parse_pair_says_which_field_failed() {
        assert_eq!(parse_pair("3", "150"), Ok((3, 150)));
        assert_eq!(parse_pair("three", "150"), Err("bad qty 'three'".to_string()));
        assert_eq!(parse_pair("3", "1,50"), Err("bad price '1,50'".to_string()));
    }
}
