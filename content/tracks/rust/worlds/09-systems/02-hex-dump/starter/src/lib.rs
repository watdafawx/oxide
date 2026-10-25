//! A hex dump: bytes written the way protocol documents write them.

/// The bytes as two-digit lowercase hex, separated by single spaces.
/// `[0x10, 0x00, 0xff]` becomes `"10 00 ff"`.
pub fn hex_dump(bytes: &[u8]) -> String {
    // TODO: two hex digits for each byte, joined with a space
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dumps_bytes_as_hex_pairs() {
        assert_eq!(hex_dump(&[0x10, 0x00, 0xff, 0x05]), "10 00 ff 05");
        assert_eq!(hex_dump(&[255, 16]), "ff 10", "255 is ff, 16 is 10");
    }

    #[test]
    fn small_bytes_keep_two_digits() {
        assert_eq!(hex_dump(&[1, 10]), "01 0a", "every byte is exactly two digits");
        assert_eq!(hex_dump(&[]), "", "no bytes, no text");
    }
}
