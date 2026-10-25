The protocol the boss speaks, the Server List Ping used by Minecraft servers, puts a length in front of every message, and most lengths are tiny. Four bytes for each would waste three almost every time. So it writes numbers as **VarInts**: small numbers take one byte, big ones up to five.

**Fixed sizes waste space.** A `u32` is always 4 bytes, even for the number 3: `00 00 00 03`. A message of 20 bytes would spend 4 bytes saying "20". A format that grows only as big as the number needs is shorter for all the small cases.

**Seven and a flag.** A VarInt splits the number into groups of **7 bits**, lowest group first. Each byte carries one group in its low 7 bits, and uses its top bit, `0x80`, as a flag: 1 means "more bytes follow", 0 means "this was the last". Numbers up to 127 fit in one byte with the flag off.

**300, by hand.** 300 in binary is `1_0010_1100`. The low 7 bits are `010_1100` (`0x2c`), and something is left, so the first byte is `0x2c | 0x80` = `0xac`. What's left is `10` (`0x02`), and nothing after it, so the second byte is `0x02`. 300 is `ac 02`.

**The tools.** `v & 0x7f` keeps the low 7 bits (`0x7f` is `0111_1111`). `v >>= 7` drops them, so the next group moves down. `byte | 0x80` sets the flag. And `loop` from the echo server repeats until nothing is left.

**Negative numbers.** Shifting a negative `i32` right copies its sign bit in from the left, so it never reaches 0 and the loop never ends. Work on `value as u32` instead: the same 32 bits read as unsigned, so `-1` becomes `0xFFFF_FFFF`, zeros shift in, and it ends after five bytes. Five 7-bit groups hold 35 bits, enough for 32.

```rust
// Not the encoder: just counts how many 7-bit groups a number needs.
fn groups(mut v: u32) -> usize {
    let mut n = 1;
    while v > 0x7f {   // more than 7 bits left
        v >>= 7;       // drop the lowest group
        n += 1;
    }
    n
}
assert_eq!(groups(300), 2);
assert_eq!(groups(-1i32 as u32), 5);
```

## Your turn

- In `encode_varint`, walk the 7-bit groups of `value as u32`, lowest first, one byte each.
- Set the top bit on every byte except the last, and give back the bytes.
