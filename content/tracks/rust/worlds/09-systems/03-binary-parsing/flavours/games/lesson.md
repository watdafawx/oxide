**Why now:** a server reply is a flat blob of bytes with a fixed header at the front. Before the pinger can read the player count, it has to read the header, and a reply cut off in the middle must give an error, not a panic.

A **cursor** is a read position over a byte slice. Every read takes some bytes and moves forward.

```rust
struct PacketCursor<'a> { data: &'a [u8], pos: usize }

impl<'a> PacketCursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], PingError> {
        if self.data.len() - self.pos < n {
            return Err(PingError::UnexpectedEof);   // check BEFORE moving
        }
        let chunk = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(chunk)
    }
}
```

- Build bigger reads from `take`: `u16::from_be_bytes([b[0], b[1]])`.
- Use `?` so the first short read ends the whole parse with `Err`.
- Check the length **before** you move the position, so a failed read leaves the cursor where it was.
- Extra bytes after the header are fine: they belong to the next field.

## Your turn

In `src/lib.rs`:

- `PacketCursor::take`, `u16_be`, `u32_be`: the reading primitives.
- `parse_packet_header`: read `kind`, `flags`, `packet_id`, `length` in that order (8 bytes, big-endian).
