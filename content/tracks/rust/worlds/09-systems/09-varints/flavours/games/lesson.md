**Why now:** every Minecraft packet starts with its length, and the ping's host and port go in the handshake, as **VarInts**. The server port 25565 would need 4 fixed bytes; a VarInt needs 3, and small numbers need one. You cannot talk to a server without this.

A VarInt writes a number 7 bits at a time, least significant group first. The top bit of each byte means "more bytes follow".

```
128  = 1000 0000          25565 = 0110 0011 1101 1101
bytes: 0x80 0x01           bytes: 0xDD 0xC7 0x01

0xDD = 1 1011101   (more follow, low 7 bits of 25565)
0xC7 = 1 1000111   (more follow, next 7 bits)
0x01 = 0 0000001   (last byte, top bit clear)
```

```rust
let mut v = value as u32;            // reinterpret: -1 becomes 0xFFFF_FFFF
let mut out = Vec::new();
loop {
    let mut byte = (v & 0x7f) as u8; // low 7 bits
    v >>= 7;
    if v != 0 { byte |= 0x80; }      // continuation bit
    out.push(byte);
    if v == 0 { break; }
}
```

- At most **5 bytes** for an `i32` (5 × 7 = 35 ≥ 32 bits). A sixth continuation byte is malformed.
- Negative numbers always take 5 bytes: `-1` is `0xFF 0xFF 0xFF 0xFF 0x0F`.
- Decoding reports how many bytes it used, because the next field starts right after.
- Truncated input (a final byte that still has the top bit set) must be an error, not a panic.

## Your turn

In `src/lib.rs`:

- `encode_varint`: the VarInt bytes for an `i32`.
- `decode_varint`: `(value, bytes used)` or a `VarIntError`.
