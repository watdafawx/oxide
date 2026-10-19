Permissions, packet flags and pixel colors all squeeze several values into one integer. Four operators do the work.

- **`|` OR** turns bits on. **`&` AND** keeps only the bits in a mask.
- **`!` NOT** flips every bit, so `x & !mask` turns the mask's bits off.
- **`<<` / `>>`** slide bits left or right. `1 << 3` is `0b1000`.

```rust
const READ: u8 = 1 << 0;   // 0b001
const WRITE: u8 = 1 << 1;  // 0b010

let mut perms = READ;
perms |= WRITE;                    // 0b011
assert!(perms & WRITE != 0);       // test a bit
perms &= !READ;                    // 0b010, cleared
assert_eq!(0b1011_0000u8.count_ones(), 3);

let color: u32 = (255 << 16) | (128 << 8) | 7;  // 0x00FF8007
assert_eq!((color >> 8) as u8, 128);            // `as u8` keeps the low 8 bits
```

Unlike C, Rust's `&` binds tighter than `==` / `!=`, so `perms & WRITE != 0` means what it looks like.

## Your turn

In `src/lib.rs`:

- `set_flag`, `clear_flag`, `has_flag`: the three one-liners above.
- `pack_rgb` / `unpack_rgb`: three `u8` into one `u32` (red in bits 16-23) and back.
- `count_set_bits`: how many bits are 1.
