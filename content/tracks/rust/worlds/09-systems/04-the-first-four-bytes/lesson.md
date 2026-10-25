Replies do not arrive as neat arrays. They arrive as a `&[u8]` of whatever length the server sent, and the checker must read a length from its first four bytes. A short or broken reply has to give "nothing there", not crash the checker.

**Arrays and slices.** `u32::from_be_bytes` wants an array, `[u8; 4]`: exactly four bytes, a size Rust knows while compiling. A reply is a slice, `&[u8]`, whose length is only known when it arrives. You need to get from one to the other.

**Never index blindly.** `&bytes[..4]` panics when the reply has only 3 bytes. `bytes.get(..4)` asks instead: it gives `Some` of the first four bytes, or `None` when there aren't four. It is the same `get` you used on lists in world 03, here with a range.

**A conversion that can fail.** `try_into()` turns a slice into an array. It gives a `Result`, because the lengths might not match: a 4-long slice becomes a `[u8; 4]`, anything else is an `Err`. Rust picks the target from the type you write after `let`.

**Chain it with `?`.** In a function that returns `Option`, `get(..4)?` leaves early with `None`. `try_into()` gives a `Result`, so `.ok()` first turns it into an `Option` (world 03), and then `?` works on it too.

```rust
fn read_port(bytes: &[u8]) -> Option<u16> {
    let two: [u8; 2] = bytes.get(..2)?.try_into().ok()?;
    Some(u16::from_be_bytes(two))
}
assert_eq!(read_port(&[0x1f, 0x90, 0xff]), Some(8080));
assert_eq!(read_port(&[0x1f]), None);
```

**For the curious.** `try_into` comes from the `TryFrom` and `TryInto` traits, the "might fail" cousins of `From` and `Into` from world 04. `u8::try_from(300)` is the same idea for a number that might not fit.

## Your turn

- In `read_u32_le`, read the **first** 4 bytes of the slice as a little-endian `u32`.
- In `read_u32_be`, the same, big-endian.
- Both give `None` when the slice has fewer than 4 bytes.
