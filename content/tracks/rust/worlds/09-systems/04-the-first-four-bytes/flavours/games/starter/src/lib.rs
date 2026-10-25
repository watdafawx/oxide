//! Reading a number from the start of a byte slice, safely.

/// Reads a little-endian u32 from the first 4 bytes of `reply`. `None` if there are fewer than 4.
pub fn read_protocol_le(reply: &[u8]) -> Option<u32> {
    // TODO: the first 4 bytes as a [u8; 4] (None if there are not 4), then the number, small end first
    todo!()
}

/// Reads a big-endian u32 from the first 4 bytes of `reply`. `None` if there are fewer than 4.
pub fn read_protocol_be(reply: &[u8]) -> Option<u32> {
    // TODO: the same, big end first
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_decodes_from_slices() {
        assert_eq!(read_protocol_le(&[0xfb, 0x02, 0x00, 0x00]), Some(763));
        assert_eq!(read_protocol_be(&[0x00, 0x00, 0x02, 0xfb]), Some(763));
        assert_eq!(
            read_protocol_le(&[1, 0, 0, 0, 99, 99]),
            Some(1),
            "extra bytes after the first 4 are ignored"
        );
    }

    #[test]
    fn short_reply_gives_none() {
        assert_eq!(read_protocol_le(&[1, 2, 3]), None);
        assert_eq!(read_protocol_be(&[]), None);
    }
}
