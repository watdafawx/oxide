//! VarInt: a number written 7 bits per byte, so small numbers take few bytes.

/// The VarInt bytes for `value`: lowest 7-bit group first, top bit set on every
/// byte except the last.
pub fn encode_varint(value: i32) -> Vec<u8> {
    // TODO: work on the bits of value as a u32. Again and again: cut off the
    //       lowest 7 bits as one byte and drop them from the number; if
    //       anything is left, set that byte's top bit (more follows). Push
    //       the byte; when nothing is left, give back the bytes.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint_bytes_match_the_protocol_table() {
        assert_eq!(encode_varint(0), [0x00]);
        assert_eq!(encode_varint(1), [0x01]);
        assert_eq!(encode_varint(127), [0x7f], "the biggest one-byte VarInt");
        assert_eq!(encode_varint(128), [0x80, 0x01]);
        assert_eq!(encode_varint(300), [0xac, 0x02]);
        assert_eq!(encode_varint(25565), [0xdd, 0xc7, 0x01]);
        assert_eq!(encode_varint(2_147_483_647), [0xff, 0xff, 0xff, 0xff, 0x07]);
    }

    #[test]
    fn negative_numbers_take_five_bytes() {
        assert_eq!(encode_varint(-1), [0xff, 0xff, 0xff, 0xff, 0x0f]);
        assert_eq!(encode_varint(i32::MIN), [0x80, 0x80, 0x80, 0x80, 0x08]);
    }
}
