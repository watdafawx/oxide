**Why now:** the server pinger sends a protocol version and a port as raw bytes, and the server rejects them unless you put the big end first.

Network packets are just bytes. The first thing every parser must know: **which byte comes first?**

A `u32` is 4 bytes. **Endianness** is the order you write them down:

- **little-endian** (LE): the small end first. x86, ARM and most file formats.
- **big-endian** (BE): the big end first. Also called "network order".

```rust
let n: u32 = 0x1234_5678;
assert_eq!(n.to_le_bytes(), [0x78, 0x56, 0x34, 0x12]);
assert_eq!(n.to_be_bytes(), [0x12, 0x34, 0x56, 0x78]);
assert_eq!(u32::from_be_bytes([0x12, 0x34, 0x56, 0x78]), n);

// From a slice: check the length first, then convert to an array.
let data: &[u8] = &[0x78, 0x56, 0x34, 0x12, 0xff];
let first: [u8; 4] = data[..4].try_into().unwrap();
assert_eq!(u32::from_le_bytes(first), n);
```

**`from_*_bytes` wants an array `[u8; 4]`, not a slice.** `try_into()` converts a 4-long slice into one, and fails if the length is wrong. `slice.get(..4)` returns `None` instead of panicking when the slice is short.

In a function that returns `Option`, chain them: `let first: [u8; 4] = data.get(..4)?.try_into().ok()?;` (`get(..4)?` leaves early with `None` when short; `try_into()` is a `Result`, so `.ok()?` turns its `Err` into the same early `None`).

## Your turn

In `src/lib.rs`:

- `to_le`, `to_be`: one-liners.
- `read_u32_le`, `read_u32_be`: read the **first** 4 bytes of the slice, `None` if there are fewer.
