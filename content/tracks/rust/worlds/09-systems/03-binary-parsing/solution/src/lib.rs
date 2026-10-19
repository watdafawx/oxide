//! Parsing a fixed binary header with a cursor.

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedEof,
}

/// A read position over a byte slice. It only moves forward.
pub struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Cursor { data, pos: 0 }
    }

    /// How many bytes are left to read.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    /// Returns the next `n` bytes and moves forward. If fewer than `n` are
    /// left, returns `UnexpectedEof` and leaves the position unchanged.
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], ParseError> {
        if self.remaining() < n {
            return Err(ParseError::UnexpectedEof);
        }
        let chunk = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(chunk)
    }

    pub fn u8(&mut self) -> Result<u8, ParseError> {
        Ok(self.take(1)?[0])
    }

    pub fn u16_be(&mut self) -> Result<u16, ParseError> {
        let b = self.take(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }

    pub fn u32_be(&mut self) -> Result<u32, ParseError> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
}

/// An 8-byte header, all fields big-endian: version, flags, id, length.
#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub version: u8,
    pub flags: u8,
    pub id: u16,
    pub length: u32,
}

/// Parses a header from the start of `data`. Extra bytes after it are ignored.
pub fn parse_header(data: &[u8]) -> Result<Header, ParseError> {
    let mut c = Cursor::new(data);
    Ok(Header {
        version: c.u8()?,
        flags: c.u8()?,
        id: c.u16_be()?,
        length: c.u32_be()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_reads_in_order() {
        let mut c = Cursor::new(&[9, 0x01, 0x02, 0, 0, 1, 0]);
        assert_eq!(c.u8(), Ok(9));
        assert_eq!(c.u16_be(), Ok(0x0102));
        assert_eq!(c.u32_be(), Ok(256));
        assert_eq!(c.remaining(), 0);
    }

    #[test]
    fn short_read_is_error() {
        let mut c = Cursor::new(&[1, 2, 3]);
        assert_eq!(c.u32_be(), Err(ParseError::UnexpectedEof));
        assert_eq!(c.remaining(), 3, "a failed read must not move the cursor");
        assert_eq!(c.u16_be(), Ok(0x0102));
        assert_eq!(c.u16_be(), Err(ParseError::UnexpectedEof));
    }

    #[test]
    fn parse_header_reads_fields() {
        let bytes = [2, 0b1000_0001, 0x12, 0x34, 0, 0, 0x01, 0x00, 0xaa, 0xbb];
        assert_eq!(
            parse_header(&bytes),
            Ok(Header { version: 2, flags: 0b1000_0001, id: 0x1234, length: 256 })
        );
    }

    #[test]
    fn parse_header_short_input_errors() {
        assert_eq!(parse_header(&[]), Err(ParseError::UnexpectedEof));
        assert_eq!(
            parse_header(&[1, 2, 3, 4, 5, 6, 7]),
            Err(ParseError::UnexpectedEof),
            "7 bytes is one too few"
        );
    }
}
