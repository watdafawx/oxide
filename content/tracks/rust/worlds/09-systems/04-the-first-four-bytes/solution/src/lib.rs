//! Reading a number from the start of a byte slice, safely.

/// Reads a little-endian u32 from the first 4 bytes of `bytes`. `None` if there are fewer than 4.
pub fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    let first: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_le_bytes(first))
}

/// Reads a big-endian u32 from the first 4 bytes of `bytes`. `None` if there are fewer than 4.
pub fn read_u32_be(bytes: &[u8]) -> Option<u32> {
    let first: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_be_bytes(first))
}

#[cfg(test)]
mod tests {
    use super::*;

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
