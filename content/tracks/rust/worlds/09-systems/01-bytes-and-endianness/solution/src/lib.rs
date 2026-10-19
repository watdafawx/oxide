//! Bytes and endianness.

/// `n` as 4 bytes, least significant byte first (little-endian).
pub fn to_le(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

/// `n` as 4 bytes, most significant byte first (big-endian).
pub fn to_be(n: u32) -> [u8; 4] {
    n.to_be_bytes()
}

/// Reads a little-endian u32 from the first 4 bytes. `None` if fewer than 4 bytes.
pub fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    let first: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_le_bytes(first))
}

/// Reads a big-endian u32 from the first 4 bytes. `None` if fewer than 4 bytes.
pub fn read_u32_be(bytes: &[u8]) -> Option<u32> {
    let first: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_be_bytes(first))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_both_endians() {
        assert_eq!(to_le(0x1234_5678), [0x78, 0x56, 0x34, 0x12]);
        assert_eq!(to_be(0x1234_5678), [0x12, 0x34, 0x56, 0x78]);
        assert_eq!(to_le(1), [1, 0, 0, 0]);
    }

    #[test]
    fn decodes_from_slices() {
        assert_eq!(read_u32_le(&[0x78, 0x56, 0x34, 0x12]), Some(0x1234_5678));
        assert_eq!(read_u32_be(&[0x12, 0x34, 0x56, 0x78]), Some(0x1234_5678));
        assert_eq!(
            read_u32_le(&[1, 0, 0, 0, 99, 99]),
            Some(1),
            "extra bytes after the first 4 are ignored"
        );
    }

    #[test]
    fn short_slice_gives_none() {
        assert_eq!(read_u32_le(&[1, 2, 3]), None);
        assert_eq!(read_u32_be(&[]), None);
    }
}
