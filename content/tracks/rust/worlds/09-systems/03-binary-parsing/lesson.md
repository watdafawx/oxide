**Why now:** the server pinger has to read the server's reply field by field from a byte stream, and a short or hostile reply must give an error, not a crash.

Every protocol starts with a **header**: a fixed run of bytes you read field by field. Reading past the end must be an error, never a panic or garbage.

The pattern is a **cursor**: a slice plus a position that only moves forward.

```rust
struct Cursor<'a> { data: &'a [u8], pos: usize }

impl<'a> Cursor<'a> {
    // (this quest returns Result<_, ParseError> instead of Option: same checks, `Err(ParseError::UnexpectedEof)` for None)
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let chunk = self.data.get(self.pos..end)?; // None if too short
        self.pos = end;                            // only advance on success
        Some(chunk)
    }
}

let mut c = Cursor { data: &[0, 7, 1], pos: 0 };
let b = c.take(2).unwrap();
assert_eq!(u16::from_be_bytes([b[0], b[1]]), 7);
assert!(c.take(5).is_none());
```

- Build every reader (`u8`, `u16_be`, ...) on one `take`.
- `?` returns early with the error, so the header parser reads like the spec.
- `?` works inside a struct literal too: `Header { a: c.u8()?, b: c.u16_be()? }` builds the struct field by field, in the order written, and returns early on the first `Err`.
- This header is **big-endian**: `version: u8, flags: u8, id: u16, length: u32` (8 bytes).

## Your turn

In `src/lib.rs`:

- `Cursor::take`: the bounds-checked core. Error is `ParseError::UnexpectedEof`.
- `u8`, `u16_be`, `u32_be`: call `take` and convert.
- `parse_header`: read the four fields in order.
