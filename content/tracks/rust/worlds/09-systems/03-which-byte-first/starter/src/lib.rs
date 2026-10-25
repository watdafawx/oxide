//! Byte order: the same number written small end first or big end first.

/// `n` as 4 bytes, least significant byte first (little-endian).
pub fn to_le(n: u32) -> [u8; 4] {
    // TODO: the 4 bytes of n, small end first
    todo!()
}

/// `n` as 4 bytes, most significant byte first (big-endian, network order).
pub fn to_be(n: u32) -> [u8; 4] {
    // TODO: the 4 bytes of n, big end first
    todo!()
}

/// The u32 whose little-endian bytes are `bytes`.
pub fn from_le(bytes: [u8; 4]) -> u32 {
    // TODO: the number these 4 bytes stand for, small end first
    todo!()
}

/// The u32 whose big-endian bytes are `bytes`.
pub fn from_be(bytes: [u8; 4]) -> u32 {
    // TODO: the number these 4 bytes stand for, big end first
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_both_endians() {
        assert_eq!(to_le(0x1234_5678), [0x78, 0x56, 0x34, 0x12]);
        assert_eq!(to_be(0x1234_5678), [0x12, 0x34, 0x56, 0x78]);
        assert_eq!(to_be(300), [0x00, 0x00, 0x01, 0x2c]);
        assert_eq!(to_le(1), [1, 0, 0, 0]);
    }

    #[test]
    fn decodes_from_arrays() {
        assert_eq!(from_le([0x78, 0x56, 0x34, 0x12]), 0x1234_5678);
        assert_eq!(from_be([0x12, 0x34, 0x56, 0x78]), 0x1234_5678);
        assert_eq!(from_be([0x00, 0x00, 0x01, 0x2c]), 300);
        assert_eq!(from_le([0x00, 0x00, 0x01, 0x2c]), 738_263_040, "the wrong order gives a different number");
    }
}
