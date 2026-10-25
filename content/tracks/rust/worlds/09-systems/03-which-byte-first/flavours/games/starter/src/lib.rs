//! Byte order: the same number written small end first or big end first.

/// `n` as 4 bytes, least significant byte first (little-endian).
pub fn protocol_to_le(n: u32) -> [u8; 4] {
    // TODO: the 4 bytes of n, small end first
    todo!()
}

/// `n` as 4 bytes, most significant byte first (big-endian, network order).
pub fn protocol_to_be(n: u32) -> [u8; 4] {
    // TODO: the 4 bytes of n, big end first
    todo!()
}

/// The u32 whose little-endian bytes are `bytes`.
pub fn protocol_from_le(bytes: [u8; 4]) -> u32 {
    // TODO: the number these 4 bytes stand for, small end first
    todo!()
}

/// The u32 whose big-endian bytes are `bytes`.
pub fn protocol_from_be(bytes: [u8; 4]) -> u32 {
    // TODO: the number these 4 bytes stand for, big end first
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_encodes_both_byte_orders() {
        assert_eq!(protocol_to_le(763), [0xfb, 0x02, 0x00, 0x00]);
        assert_eq!(protocol_to_be(763), [0x00, 0x00, 0x02, 0xfb]);
        assert_eq!(protocol_to_le(0x1234_5678), [0x78, 0x56, 0x34, 0x12]);
        assert_eq!(protocol_to_be(0x1234_5678), [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn protocol_decodes_from_arrays() {
        assert_eq!(protocol_from_le([0xfb, 0x02, 0x00, 0x00]), 763);
        assert_eq!(protocol_from_be([0x00, 0x00, 0x02, 0xfb]), 763);
        assert_eq!(
            protocol_from_le([0x00, 0x00, 0x02, 0xfb]),
            4_211_212_288,
            "the wrong order gives a version from the far future"
        );
    }
}
