//! Reading a number from the start of a byte slice, safely.

/// Reads a little-endian u32 from the first 4 bytes of `bytes`. `None` if there are fewer than 4.
pub fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    // TODO: the first 4 bytes as a [u8; 4] (None if there are not 4), then the number, small end first
    todo!()
}

/// Reads a big-endian u32 from the first 4 bytes of `bytes`. `None` if there are fewer than 4.
pub fn read_u32_be(bytes: &[u8]) -> Option<u32> {
    // TODO: the same, big end first
    todo!()
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
