//! A cursor: a byte slice plus a position that only moves forward.

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
        if self.remaining() < n {
            return Err(PingError::UnexpectedEof);
        }
        let chunk = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(chunk)
    }

    /// Reads one byte.
    pub fn u8(&mut self) -> Result<u8, PingError> {
        Ok(self.take(1)?[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_reply_bytes_in_order() {
        let mut c = PacketCursor::new(&[9, 0x01, 0x02, 7]);
        assert_eq!(c.take(1), Ok(&[9u8][..]));
        assert_eq!(c.take(2), Ok(&[0x01u8, 0x02][..]));
        assert_eq!(c.remaining(), 1);
        assert_eq!(c.u8(), Ok(7));
        assert_eq!(c.remaining(), 0);
        assert_eq!(c.take(0), Ok(&[][..]), "taking 0 bytes at the end is fine");
    }

    #[test]
    fn short_reply_is_an_error() {
        let mut c = PacketCursor::new(&[1, 2, 3]);
        assert_eq!(c.take(4), Err(PingError::UnexpectedEof));
        assert_eq!(c.remaining(), 3, "a failed read must not move the cursor");
        assert_eq!(c.take(3), Ok(&[1u8, 2, 3][..]));
        assert_eq!(c.u8(), Err(PingError::UnexpectedEof));
    }
}
