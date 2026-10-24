The answer from a server starts with a VarInt too, so the checker must also read them. Decoding is the encoder backwards: take each byte's 7 bits, put them back in their place, and stop at the byte that says "last".

**Put each group back.** The first byte holds the lowest 7 bits, the second the next 7, and so on. So byte number `i` (counting from 0) goes `7 * i` bits to the left: `u32::from(b & 0x7f) << (7 * i)`. OR it into a running `result`, and the groups land side by side without overlapping.

**Where it stops.** The first byte whose top bit is 0, `b & 0x80 == 0`, is the last one. Stop there and turn the bits back into a signed number with `result as i32`, so `0xFFFF_FFFF` becomes `-1` again.

**How many bytes.** Give back how many bytes the VarInt used as well, `i + 1`. A VarInt has no fixed size, so this is the only way the caller knows where the next field starts. Bytes after the VarInt are left alone.

**Bytes with their positions.** `for (i, &b) in bytes.iter().enumerate()` gives each byte with its position, as in world 03. The `&b` in the pattern copies the byte out of the `&u8` the iterator hands over, so `b` is a plain `u8`.

**It never ended.** If the loop runs out of bytes before it finds a last byte, the input was cut short. That is `Err(VarIntError::Truncated)` after the loop.

```rust
// 300 is `ac 02`: groups 0x2c (from 0xac) and 0x02.
let mut v: u32 = 0;
v |= u32::from(0xacu8 & 0x7f) << 0;   // lowest group, no shift
v |= u32::from(0x02u8 & 0x7f) << 7;   // next group, 7 bits up
assert_eq!(v, 300);
```

## Your turn

- In `decode_varint`, go through the bytes with their positions and put each byte's 7 bits into `result`.
- At the first byte without the top bit, give back the value and the number of bytes used.
- If the bytes run out first, give back `Truncated`.
