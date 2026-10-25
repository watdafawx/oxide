An address like `192.168.50.3` is four bytes, and many programs keep it as one `u32`: one number is easy to compare, store and send. The checker needs to pack the four parts into one number and take them out again, which means moving bits to a different place.

**Slide bits over.** `<<` shifts every bit to the left: `1 << 3` is `0b1000`, which is 8. Shifting left by 8 moves a byte up into the next byte's place, so `0x12 << 8` is `0x1200`. `>>` shifts right, and the bits that fall off the end are gone: `0x1234 >> 8` is `0x12`.

**Make room first.** A `u8` has only 8 bits, so `a << 24` on a `u8` would push every bit out, and Rust refuses it. Widen the byte first: `u32::from(a)` turns it into a `u32` with room for 32 bits. It is the `From` trait from world 04: a conversion that can never fail.

**Glue with `|`.** After shifting, each byte sits in its own place and no bits overlap. OR from the last quest then joins them into one number: `0x1200 | 0x34` is `0x1234`.

**Cut with `as u8`.** You met in world 01 that a cast can cut things off. `as u8` keeps only the lowest 8 bits and drops the rest, so `(0x1234 >> 8) as u8` is `0x12` and `0x1234 as u8` is `0x34`. That is exactly how you pick one byte out.

```rust
// Two bytes into a u16 and back.
let (hi, lo): (u8, u8) = (0x1f, 0x90);
let port = (u16::from(hi) << 8) | u16::from(lo);   // 0x1f90 = 8080
assert_eq!((port >> 8) as u8, 0x1f);
assert_eq!(port as u8, 0x90);
```

## Your turn

- In `pack_ipv4`, put `a` in the top byte and `d` in the lowest byte of one `u32`.
- In `unpack_ipv4`, take the four bytes out again, `a` first.
