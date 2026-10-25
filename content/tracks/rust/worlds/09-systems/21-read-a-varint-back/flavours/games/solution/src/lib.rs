//! Reading a VarInt back: the mirror of the encoder.

#[derive(Debug, PartialEq, Eq)]
pub enum VarIntError {
    /// The bytes ended before a byte without the "more follows" bit.
    Truncated,
}

/// The VarInt bytes for `value` (your encoder from the last quest).
pub fn encode_varint(value: i32) -> Vec<u8> {
    let mut v = value as u32;
    let mut out = Vec::new();
    loop {
        let mut byte = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if v == 0 {
            return out;
        }
    }
}

/// Decodes one VarInt from the start of `bytes`.
/// Returns the value and how many bytes it used. Later bytes are ignored.
pub fn decode_varint(bytes: &[u8]) -> Result<(i32, usize), VarIntError> {
    let mut result: u32 = 0;
    for (i, &b) in bytes.iter().enumerate() {
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
    fn decode_returns_value_and_bytes_used() {
        assert_eq!(decode_varint(&[0x00]), Ok((0, 1)));
        assert_eq!(decode_varint(&[0xac, 0x02]), Ok((300, 2)));
        assert_eq!(decode_varint(&[0xdd, 0xc7, 0x01]), Ok((25565, 3)));
        assert_eq!(
            decode_varint(&[0x80, 0x01, 0xaa, 0xbb]),
            Ok((128, 2)),
            "bytes after the VarInt belong to the next field"
        );
        assert_eq!(decode_varint(&[0xff, 0xff, 0xff, 0xff, 0x0f]), Ok((-1, 5)));
        assert_eq!(decode_varint(&[0x80, 0x80]), Err(VarIntError::Truncated), "it never ended");
    }

    #[test]
    fn edge_values_survive_a_round_trip() {
        for v in [0, 1, 127, 128, 300, 16383, 16384, i32::MAX, -1, i32::MIN] {
            let bytes = encode_varint(v);
            assert_eq!(decode_varint(&bytes), Ok((v, bytes.len())), "value {v}");
        }
    }
}
