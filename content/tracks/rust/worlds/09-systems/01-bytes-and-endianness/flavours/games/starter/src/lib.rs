//! Bytes and endianness for the server pinger.

/// `n` as 4 bytes, least significant byte first (little-endian).
pub fn protocol_to_le(n: u32) -> [u8; 4] {
    // TODO: to_le_bytes
    todo!()
}

/// `n` as 4 bytes, most significant byte first (big-endian, network order).
pub fn protocol_to_be(n: u32) -> [u8; 4] {
    // TODO: to_be_bytes
    todo!()
}

/// Reads a little-endian u32 from the first 4 bytes of a reply. `None` if fewer than 4 bytes.
pub fn read_protocol_le(reply: &[u8]) -> Option<u32> {
    // TODO: reply.get(..4)?, try_into, from_le_bytes
    todo!()
}

/// Reads a big-endian u32 from the first 4 bytes of a reply. `None` if fewer than 4 bytes.
pub fn read_protocol_be(reply: &[u8]) -> Option<u32> {
    // TODO: reply.get(..4)?, try_into, from_be_bytes
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
