Game protocols start every packet with a **header**: a fixed run of fields with a fixed size, so the reader knows how to read the rest. This one is 8 bytes: a kind (1 byte), flags (1 byte), a packet id (2 bytes) and a body length (4 bytes), all big-endian. With the cursor, reading it becomes one line per field.

**Bigger reads from `take`.** `let b = self.take(2)?;` gives exactly two bytes or leaves early with the error. Then `[b[0], b[1]]` builds the array `from_be_bytes` wants. Indexing is safe here, because `take` has already checked that the two bytes exist.

**A header is a struct.** Each field of the header becomes a field of a Rust struct with the matching type: `u8`, `u8`, `u16`, `u32`. 1 + 1 + 2 + 4 is the 8 bytes.

**`?` inside a struct literal.** You can write `?` on each field value: `PacketHeader { kind: c.u8()?, ... }`. Rust runs the fields in the order you wrote them, and the first `Err` ends the whole function with that error. The parser now reads like the protocol wiki's table.

**Extra bytes are fine.** The header only reads its 8 bytes. Whatever follows is the packet body, the next part's job.

```rust
pub struct ServerVersion {
    pub major: u8,
    pub minor: u16,
}

pub fn parse_server_version(data: &[u8]) -> Result<ServerVersion, PingError> {
    let mut c = PacketCursor::new(data);
    Ok(ServerVersion { major: c.u8()?, minor: c.u16_be()? })
}
```

## Your turn

- In `u16_be` and `u32_be`, take 2 or 4 bytes and build a big-endian number.
- In `parse_packet_header`, make a cursor over `data` and read `kind`, `flags`, `packet_id` and `length` in that order.
