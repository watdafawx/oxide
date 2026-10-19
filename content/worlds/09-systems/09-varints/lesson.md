Minecraft packets start with a length, and most lengths are tiny. A **VarInt** stores small numbers in one byte and big ones in up to five. You need this for the boss.

The recipe (LEB128):

- Split the number into groups of **7 bits**, lowest group first.
- Each byte holds one group in its low 7 bits. The top bit (`0x80`) means "more bytes follow".
- The last byte has the top bit clear.

```rust
// 300 = 0b1_0010_1100  ->  groups: 0101100 (0x2c), 0000010 (0x02)
// byte 1: 0x2c | 0x80 = 0xac   (more to come)
// byte 2: 0x02                 (last)
fn encode(value: i32) -> Vec<u8> {
    let mut v = value as u32;     // negatives become 5-byte values
    let mut out = Vec::new();
    loop {
        let mut byte = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 { byte |= 0x80; }
        out.push(byte);
        if v == 0 { return out; }
    }
}
assert_eq!(encode(300), vec![0xac, 0x02]);
```

- `127` is `[0x7f]`, `128` is `[0x80, 0x01]`, `-1` is `[0xff, 0xff, 0xff, 0xff, 0x0f]`.
- A VarInt is **at most 5 bytes**. A 6th continuation byte means a broken or hostile packet: reject it, never loop forever.
- Decoding returns the value **and how many bytes it used**, so the caller knows where the next field starts.

## Your turn

In `src/lib.rs`:

- `encode_varint`: the loop above, appending to a `Vec`.
- `decode_varint`: the reverse. Returns `Err(VarIntError::TooLong)` or `Err(VarIntError::Truncated)` on bad input.
