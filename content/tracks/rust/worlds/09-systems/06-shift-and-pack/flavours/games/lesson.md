A server's MOTD can be coloured, and a colour is three bytes: red, green and blue. Gold is `ff aa 00`. The pinger keeps a colour as one `u32`, `0x00FFAA00`, which is easy to compare and store, so it needs to pack three bytes into one number and take them out again. That means moving bits to a different place.

**Slide bits over.** `<<` shifts every bit to the left: `1 << 3` is `0b1000`, which is 8. Shifting left by 8 moves a byte up into the next byte's place, so `0x12 << 8` is `0x1200`. `>>` shifts right, and the bits that fall off the end are gone: `0x1234 >> 8` is `0x12`.

**Make room first.** A `u8` has only 8 bits, so `r << 16` on a `u8` would push every bit out, and Rust refuses it. Widen the byte first: `u32::from(r)` turns it into a `u32` with room for 32 bits. It is the `From` trait from world 04: a conversion that can never fail.

**Glue with `|`.** After shifting, each byte sits in its own place and no bits overlap. OR from the last quest then joins them into one number: `0x1200 | 0x34` is `0x1234`.

**Cut with `as u8`.** You met in world 01 that a cast can cut things off. `as u8` keeps only the lowest 8 bits and drops the rest, so `(0x1234 >> 8) as u8` is `0x12` and `0x1234 as u8` is `0x34`. That is exactly how you pick one byte out.

```rust
// The two bytes of the server port into a u16 and back.
let (hi, lo): (u8, u8) = (0x63, 0xdd);
let port = (u16::from(hi) << 8) | u16::from(lo);   // 0x63dd = 25565
assert_eq!((port >> 8) as u8, 0x63);
assert_eq!(port as u8, 0xdd);
```

## Your turn

- In `pack_color`, put red in bits 16-23, green in 8-15 and blue in the lowest byte.
- In `unpack_color`, take the three bytes out again, red first.
