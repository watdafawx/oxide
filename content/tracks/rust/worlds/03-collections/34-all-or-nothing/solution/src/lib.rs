/// "12" -> Ok(12). Anything else -> Err("bad qty '<text>'"). (From the last quest.)
pub fn parse_qty(text: &str) -> Result<u32, String> {
    text.trim().parse::<u32>().map_err(|_| format!("bad qty '{text}'"))
}

/// Every qty, or the first error: ["12", "6"] -> Ok([12, 6]); ["12", "x"] -> Err("bad qty 'x'").
pub fn parse_all(fields: &[&str]) -> Result<Vec<u32>, String> {
    fields.iter().map(|f| parse_qty(f)).collect()
}

/// The same, but the error says which item failed, counting from 1:
/// ["12", "x"] -> Err("item 2: bad qty 'x'")
pub fn parse_all_numbered(fields: &[&str]) -> Result<Vec<u32>, String> {
    fields
        .iter()
        .enumerate()
        .map(|(i, f)| parse_qty(f).map_err(|e| format!("item {}: {e}", i + 1)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_gives_every_number_or_the_first_error() {
        assert_eq!(parse_all(&["12", " 6", "1"]), Ok(vec![12, 6, 1]));
        assert_eq!(parse_all(&["12", "x", "y"]), Err("bad qty 'x'".to_string()), "the first error wins");
    }

    #[test]
    fn parse_all_numbered_says_which_item_failed() {
        assert_eq!(parse_all_numbered(&["12", "6"]), Ok(vec![12, 6]));
        assert_eq!(parse_all_numbered(&["12", "6", "x"]), Err("item 3: bad qty 'x'".to_string()));
    }
}
