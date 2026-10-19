**Why now:** every Minecraft packet starts with a VarInt length, so the pinger cannot send or read a single packet without them.

A **VarInt** stores small numbers in one byte and big ones in up to five. You need this for the boss.

The recipe (LEB128):

- Split the number into groups of **7 bits**, lowest group first.
- Each byte holds one group in its low 7 bits. The top bit (`0x80`) means "more bytes follow"; the last byte has it clear.

```rust
// 300 = 0b1_0010_1100  ->  groups: 0101100 (0x2c), 0000010 (0x02)
// byte 1: 0x2c | 0x80 = 0xac   (more to come)
// byte 2: 0x02                 (last)

// Just the splitting, no flag bits yet: how many 7-bit groups does `v` need?
fn groups(mut v: u32) -> usize {
    let mut n = 1;
    while v >= 0x80 {   // more than 7 bits left
        v >>= 7;        // drop the lowest group
        n += 1;
    }
    n
}
assert_eq!(groups(300), 2);
```

- `127` is `[0x7f]`, `128` is `[0x80, 0x01]`, `-1` is `[0xff, 0xff, 0xff, 0xff, 0x0f]`.
- A VarInt is **at most 5 bytes**. A 6th continuation byte means a broken or hostile packet: reject it, never loop forever.

## Your turn

In `src/lib.rs`:

- `encode_varint`: walk the 7-bit groups as in `groups`, appending each byte (flag bit set on all but the last) to a `Vec`.
- `decode_varint`: the reverse; return the value **and how many bytes it used**, so the caller knows where the next field starts. Returns `Err(VarIntError::TooLong)` or `Err(VarIntError::Truncated)` on bad input.
