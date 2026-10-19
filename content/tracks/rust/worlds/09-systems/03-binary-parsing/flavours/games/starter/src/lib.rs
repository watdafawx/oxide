//! Parsing a fixed binary packet header with a cursor.

#[derive(Debug, PartialEq, Eq)]
pub enum PingError {
    UnexpectedEof,
}

/// A read position over a byte slice. It only moves forward.
pub struct PacketCursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> PacketCursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        PacketCursor { data, pos: 0 }
    }

    /// How many bytes are left to read.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    /// Returns the next `n` bytes and moves forward. If fewer than `n` are
    /// left, returns `UnexpectedEof` and leaves the position unchanged.
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], PingError> {
        // TODO: check remaining() first, then slice and advance pos
        todo!()
    }

    pub fn u8(&mut self) -> Result<u8, PingError> {
        Ok(self.take(1)?[0])
    }

    pub fn u16_be(&mut self) -> Result<u16, PingError> {
        // TODO: take(2)?, then u16::from_be_bytes
        todo!()
    }

    pub fn u32_be(&mut self) -> Result<u32, PingError> {
        // TODO: take(4)?, then u32::from_be_bytes
        todo!()
    }
}

/// An 8-byte packet header, all fields big-endian: kind, flags, packet id, body length.
#[derive(Debug, PartialEq, Eq)]
pub struct PacketHeader {
    pub kind: u8,
    pub flags: u8,
    pub packet_id: u16,
    pub length: u32,
}

/// Parses a header from the start of `data`. Extra bytes after it are ignored.
pub fn parse_packet_header(data: &[u8]) -> Result<PacketHeader, PingError> {
    // TODO: PacketCursor::new, then read the four fields in order with ?
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_reads_fields_in_order() {
        let mut c = PacketCursor::new(&[9, 0x01, 0x02, 0, 0, 1, 0]);
        assert_eq!(c.u8(), Ok(9));
        assert_eq!(c.u16_be(), Ok(0x0102));
        assert_eq!(c.u32_be(), Ok(256));
        assert_eq!(c.remaining(), 0);
    }

    #[test]
    fn short_read_is_an_error() {
        let mut c = PacketCursor::new(&[1, 2, 3]);
        assert_eq!(c.u32_be(), Err(PingError::UnexpectedEof));
        assert_eq!(c.remaining(), 3, "a failed read must not move the cursor");
        assert_eq!(c.u16_be(), Ok(0x0102));
        assert_eq!(c.u16_be(), Err(PingError::UnexpectedEof));
    }

    #[test]
    fn parse_packet_header_reads_fields() {
        let bytes = [2, 0b1000_0001, 0x12, 0x34, 0, 0, 0x01, 0x00, 0xaa, 0xbb];
        assert_eq!(
            parse_packet_header(&bytes),
            Ok(PacketHeader { kind: 2, flags: 0b1000_0001, packet_id: 0x1234, length: 256 })
        );
    }

    #[test]
    fn parse_packet_header_short_reply_errors() {
        assert_eq!(parse_packet_header(&[]), Err(PingError::UnexpectedEof));
        assert_eq!(
            parse_packet_header(&[1, 2, 3, 4, 5, 6, 7]),
            Err(PingError::UnexpectedEof),
            "7 bytes is one too few"
        );
    }
}
