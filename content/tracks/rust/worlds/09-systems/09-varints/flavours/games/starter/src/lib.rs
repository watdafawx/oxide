//! Minecraft-style VarInt (LEB128, i32, at most 5 bytes).

#[derive(Debug, PartialEq, Eq)]
pub enum VarIntError {
    /// The input ended before a byte without the continuation bit.
    Truncated,
    /// More than 5 bytes with the continuation bit set.
    TooLong,
}

/// Returns the VarInt bytes for `value`.
pub fn encode_varint(value: i32) -> Vec<u8> {
    // TODO: 7 bits per byte, least significant group first, 0x80 = more follow
    todo!()
}

/// Decodes one VarInt from the start of `bytes`.
/// Returns the value and how many bytes it used. Later bytes are ignored.
pub fn decode_varint(bytes: &[u8]) -> Result<(i32, usize), VarIntError> {
    // TODO: accumulate 7 bits per byte; Err(TooLong) after 5 bytes, Err(Truncated) if the bytes run out
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint_bytes_match_the_protocol_table() {
        assert_eq!(encode_varint(0), [0x00]);
        assert_eq!(encode_varint(1), [0x01]);
        assert_eq!(encode_varint(127), [0x7f]);
        assert_eq!(encode_varint(128), [0x80, 0x01]);
        assert_eq!(encode_varint(255), [0xff, 0x01]);
        assert_eq!(encode_varint(25565), [0xdd, 0xc7, 0x01]);
        assert_eq!(encode_varint(2_147_483_647), [0xff, 0xff, 0xff, 0xff, 0x07]);
        assert_eq!(encode_varint(-1), [0xff, 0xff, 0xff, 0xff, 0x0f]);
    }

    #[test]
    fn decode_returns_value_and_bytes_used() {
        assert_eq!(decode_varint(&[0x00]), Ok((0, 1)));
        assert_eq!(decode_varint(&[0xdd, 0xc7, 0x01]), Ok((25565, 3)));
        assert_eq!(
            decode_varint(&[0x80, 0x01, 0xaa, 0xbb]),
            Ok((128, 2)),
            "bytes after the VarInt belong to the next field"
        );
        assert_eq!(decode_varint(&[0xff, 0xff, 0xff, 0xff, 0x0f]), Ok((-1, 5)));
    }

    #[test]
    fn edge_values_survive_a_round_trip() {
        for v in [0, 1, 127, 128, 300, 16383, 16384, i32::MAX, -1, i32::MIN] {
            let bytes = encode_varint(v);
            assert!(bytes.len() <= 5);
            assert_eq!(decode_varint(&bytes), Ok((v, bytes.len())), "value {v}");
        }
    }

    #[test]
    fn broken_packets_give_errors() {
        assert_eq!(decode_varint(&[]), Err(VarIntError::Truncated));
        assert_eq!(decode_varint(&[0x80, 0x80]), Err(VarIntError::Truncated));
        assert_eq!(
            decode_varint(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x01]),
            Err(VarIntError::TooLong),
            "a 6th byte is never valid"
        );
    }
}
