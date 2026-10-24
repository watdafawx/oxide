/// "12" -> Ok(12). Anything else -> Err("bad qty '<text>'").
pub fn parse_qty(text: &str) -> Result<u32, String> {
    text.trim().parse::<u32>().map_err(|_| format!("bad qty '{text}'"))
}

/// ("3", "150") -> Ok((3, 150)). The error says which field was bad:
/// "bad qty 'x'" or "bad price 'y'".
pub fn parse_pair(qty: &str, price: &str) -> Result<(u32, u32), String> {
    let qty_n = qty.trim().parse::<u32>().map_err(|_| format!("bad qty '{qty}'"))?;
    let price_n = price.trim().parse::<u32>().map_err(|_| format!("bad price '{price}'"))?;
    Ok((qty_n, price_n))
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
