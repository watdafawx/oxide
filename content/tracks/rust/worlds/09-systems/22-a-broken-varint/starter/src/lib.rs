//! A VarInt reader that survives broken and hostile input.

#[derive(Debug, PartialEq, Eq)]
pub enum VarIntError {
    /// The bytes ended before a byte without the "more follows" bit.
    Truncated,
    /// More than 5 bytes with the "more follows" bit set.
    TooLong,
}

/// Decodes one VarInt from the start of `bytes`.
/// Returns the value and how many bytes it used. Later bytes are ignored.
pub fn decode_varint(bytes: &[u8]) -> Result<(i32, usize), VarIntError> {
    let mut result: u32 = 0;
    for (i, &b) in bytes.iter().enumerate() {
        // TODO: a VarInt is at most 5 bytes. Before using byte number i, stop
        //       with TooLong if it would be a 6th one.
        result |= u32::from(b & 0x7f) << (7 * i);
        if b & 0x80 == 0 {
            return Ok((result as i32, i + 1));
        }
    }
    Err(VarIntError::Truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_input_errors() {
        assert_eq!(decode_varint(&[]), Err(VarIntError::Truncated));
        assert_eq!(decode_varint(&[0x80, 0x80]), Err(VarIntError::Truncated));
        assert_eq!(
            decode_varint(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x01]),
            Err(VarIntError::TooLong),
            "a 6th byte is never valid"
        );
        assert_eq!(decode_varint(&[0xff; 100]), Err(VarIntError::TooLong), "endless continuation bytes");
        assert_eq!(decode_varint(&[0xff, 0xff, 0xff, 0xff, 0x0f]), Ok((-1, 5)), "5 bytes is still fine");
    }
}
