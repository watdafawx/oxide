/// "64" -> Ok(64). Anything else -> Err("bad count '<text>'"). (From the last quest.)
pub fn parse_count(text: &str) -> Result<u32, String> {
    text.trim().parse::<u32>().map_err(|_| format!("bad count '{text}'"))
}

/// Every count, or the first error: ["1", "16"] -> Ok([1, 16]); ["1", "x"] -> Err("bad count 'x'").
pub fn parse_all(fields: &[&str]) -> Result<Vec<u32>, String> {
    fields.iter().map(|f| parse_count(f)).collect()
}

/// The same, but the error says which slot failed, counting from 1:
/// ["1", "x"] -> Err("slot 2: bad count 'x'")
pub fn parse_all_numbered(fields: &[&str]) -> Result<Vec<u32>, String> {
    fields
        .iter()
        .enumerate()
        .map(|(i, f)| parse_count(f).map_err(|e| format!("slot {}: {e}", i + 1)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_gives_every_number_or_the_first_error() {
        assert_eq!(parse_all(&["1", " 16", "8"]), Ok(vec![1, 16, 8]));
        assert_eq!(parse_all(&["1", "x", "y"]), Err("bad count 'x'".to_string()), "the first error wins");
    }

    #[test]
    fn parse_all_numbered_says_which_slot_failed() {
        assert_eq!(parse_all_numbered(&["1", "16"]), Ok(vec![1, 16]));
        assert_eq!(parse_all_numbered(&["1", "16", "x"]), Err("slot 3: bad count 'x'".to_string()));
    }
}
