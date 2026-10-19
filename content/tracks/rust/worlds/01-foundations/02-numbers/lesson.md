Health bars, damage, scores: all numbers, and Rust wants to know exactly which kind.

```rust
let hp: u8 = 200;          // 0..=255
let level: i32 = -3;       // signed 32-bit: the default integer
let speed = 1.5;           // f64: the default float
let half = level as f64 / 2.0;   // `as` converts between number types
let wrapped = hp.wrapping_add(100);  // 44, no crash
```

- Integers: `i8 i16 i32 i64` (signed), `u8 u16 u32 u64` (unsigned), `usize` for sizes and indexes.
- Floats: `f32`, `f64`. Integers and floats **never mix** without an `as`.
- `as` can **truncate**: `300 as u8` keeps only the low 8 bits (44).
- In a debug build, `250u8 + 10` **panics** (overflow). `wrapping_add` says "wrap on purpose".

## Your turn

- Open `src/lib.rs`.
- `average`: add two `i32`s and return the exact average as `f64`.
- `low_byte`: cast a `u32` to `u8`.
- `wrap_add`: add two `u8`s without ever panicking.
