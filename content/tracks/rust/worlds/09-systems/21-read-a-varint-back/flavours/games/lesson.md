The server's answer starts with a VarInt length too, so the pinger must also read them. Decoding is the encoder backwards: take each byte's 7 bits, put them back in their place, and stop at the byte that says "last".

**Put each group back.** The first byte holds the lowest 7 bits, the second the next 7, and so on. So byte number `i` (counting from 0) goes `7 * i` bits to the left: `u32::from(b & 0x7f) << (7 * i)`. OR it into a running `result`, and the groups land side by side without overlapping.

**Where it stops.** The first byte whose top bit is 0, `b & 0x80 == 0`, is the last one. Stop there and turn the bits back into a signed number with `result as i32`, so `0xFFFF_FFFF` becomes `-1` again.

**How many bytes.** Give back how many bytes the VarInt used as well, `i + 1`. A VarInt has no fixed size, so this is the only way the pinger knows where the next field of the packet starts. Bytes after the VarInt are left alone.

**Bytes with their positions.** `for (i, &b) in bytes.iter().enumerate()` gives each byte with its position, as in world 03. The `&b` in the pattern copies the byte out of the `&u8` the iterator hands over, so `b` is a plain `u8`.

**It never ended.** If the loop runs out of bytes before it finds a last byte, the packet was cut short. That is `Err(VarIntError::Truncated)` after the loop.

```rust
// 25565 is `dd c7 01`: here are its first two groups.
let mut v: u32 = 0;
v |= u32::from(0xddu8 & 0x7f) << 0;   // lowest group, no shift
v |= u32::from(0xc7u8 & 0x7f) << 7;   // next group, 7 bits up
assert_eq!(v, 9181);                  // the third byte adds 1 << 14 = 16384
```

## Your turn

- In `decode_varint`, go through the bytes with their positions and put each byte's 7 bits into `result`.
- At the first byte without the top bit, give back the value and the number of bytes used.
- If the bytes run out first, give back `Truncated`.
