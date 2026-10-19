**Why now:** a VarInt, the length format of every Minecraft packet, is built from 7-bit groups and a "more bytes follow" flag; you need shifts and masks before you can write one.

Packet flags and VarInt bytes squeeze several values into one integer. Four operators do the work.

- **`|` OR** turns bits on. **`&` AND** keeps only the bits in a mask.
- **`!` NOT** flips every bit, so `x & !mask` turns the mask's bits off.
- **`<<` / `>>`** slide bits left or right. `1 << 3` is `0b1000`.

```rust
const COMPRESSED: u8 = 1 << 0;   // 0b001
const ENCRYPTED: u8 = 1 << 1;    // 0b010

let mut flags = COMPRESSED;
flags |= ENCRYPTED;                // 0b011
assert!(flags & ENCRYPTED != 0);   // test a bit
flags &= !COMPRESSED;              // 0b010, cleared
assert_eq!(0b1011_0000u8.count_ones(), 3);

let color: u32 = (255 << 16) | (128 << 8) | 7;  // 0x00FF8007
assert_eq!((color >> 8) as u8, 128);            // `as u8` keeps the low 8 bits
```

Unlike C, Rust's `&` binds tighter than `==` / `!=`, so `flags & ENCRYPTED != 0` means what it looks like.

## Your turn

In `src/lib.rs`:

- `set_flag`, `clear_flag`, `has_flag`: the three one-liners above.
- `pack_rgb` / `unpack_rgb`: three `u8` into one `u32` (red in bits 16-23) and back.
- `count_set_bits`: how many bits are 1.
