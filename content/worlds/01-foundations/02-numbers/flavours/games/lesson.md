**Why now:** a stack holds 0 to 64 items, never -3. That is why a stack count is a `u32` (unsigned, no negatives), not an `i32`. Your calculator will mix counts, averages and ids, and Rust wants to know exactly which number type each one is.

```rust
let count: u32 = 64;       // 0..=4_294_967_295, never negative
let delta: i32 = -3;       // signed 32-bit: the default integer
let ratio = 1.5;           // f64: the default float
let half = count as f64 / 2.0;   // `as` converts between number types
let counter: u8 = 200;
let wrapped = counter.wrapping_add(100);  // 44, no crash
```

- Integers: `i8 i16 i32 i64` (signed), `u8 u16 u32 u64` (unsigned), `usize` for sizes and indexes.
- Floats: `f32`, `f64`. Integers and floats **never mix** without an `as`.
- `as` can **truncate**: `300 as u8` keeps only the low 8 bits (44).
- In a debug build, `250u8 + 10` **panics** (overflow). `wrapping_add` says "wrap on purpose".

## Your turn

- Open `src/lib.rs`.
- `average_stack`: add two `i32`s and return the exact average as `f64`.
- `slot_byte`: cast a `u32` to `u8`.
- `wrap_count`: add two `u8`s without ever panicking.
