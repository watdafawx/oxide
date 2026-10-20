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
// Just the splitting, no flag bits yet: how many 7-bit groups does `v` need?
fn groups(mut v: u32) -> usize {
    let mut n = 1;
    while v >= 0x80 {   // more than 7 bits left
        v >>= 7;        // drop the lowest group
        n += 1;
    }
    n
}
assert_eq!(groups(25565), 3);

let low7 = (25565u32 & 0x7f) as u8;   // the lowest group: 0x5d
let with_flag = low7 | 0x80;          // set the "more follow" bit: 0xdd
```

- `value as u32` reinterprets an `i32` so its bits survive: `-1` becomes `0xFFFF_FFFF`. Split that, not the signed number. Decoding ends with `result as i32`.
- Decoding is the mirror: take each byte's low 7 bits (`b & 0x7f`), shift them left by `7 * position`, OR them into a `u32`; stop at the first byte whose top bit (`b & 0x80`) is 0. `for (i, &b) in bytes.iter().enumerate()` gives you the position `i` and the byte.
- At most **5 bytes** for an `i32` (5 × 7 = 35 ≥ 32 bits). A sixth continuation byte is malformed.
- Negative numbers always take 5 bytes: `-1` is `0xFF 0xFF 0xFF 0xFF 0x0F`.
- Decoding reports how many bytes it used, because the next field starts right after.
- Truncated input (a final byte that still has the top bit set) must be an error, not a panic.

## Your turn

In `src/lib.rs`:

- `encode_varint`: the VarInt bytes for an `i32`.
- `decode_varint`: `(value, bytes used)` or a `VarIntError`.
