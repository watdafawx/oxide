Server replies do not arrive as neat arrays. They arrive as a `&[u8]` of whatever length the server sent, and the pinger reads the protocol version from the first four bytes. A modded server with a broken plugin can send a reply cut short, and that must give "nothing there", not crash your launcher.

**Arrays and slices.** `u32::from_be_bytes` wants an array, `[u8; 4]`: exactly four bytes, a size Rust knows while compiling. A reply is a slice, `&[u8]`, whose length is only known when it arrives. You need to get from one to the other.

**Never index blindly.** `&reply[..4]` panics when the reply has only 3 bytes. `reply.get(..4)` asks instead: it gives `Some` of the first four bytes, or `None` when there aren't four. It is the same `get` you used on lists in world 03, here with a range.

**A conversion that can fail.** `try_into()` turns a slice into an array. It gives a `Result`, because the lengths might not match: a 4-long slice becomes a `[u8; 4]`, anything else is an `Err`. Rust picks the target from the type you write after `let`.

**Chain it with `?`.** In a function that returns `Option`, `get(..4)?` leaves early with `None`. `try_into()` gives a `Result`, so `.ok()` first turns it into an `Option` (world 03), and then `?` works on it too.

```rust
fn read_port(reply: &[u8]) -> Option<u16> {
    let two: [u8; 2] = reply.get(..2)?.try_into().ok()?;
    Some(u16::from_be_bytes(two))
}
assert_eq!(read_port(&[0x63, 0xdd, 0xff]), Some(25565));
assert_eq!(read_port(&[0x63]), None);
```

**For the curious.** `try_into` comes from the `TryFrom` and `TryInto` traits, the "might fail" cousins of `From` and `Into` from world 04. `u8::try_from(300)` is the same idea for a number that might not fit.

## Your turn

- In `read_protocol_le`, read the **first** 4 bytes of the reply as a little-endian `u32`.
- In `read_protocol_be`, the same, big-endian.
- Both give `None` when the reply has fewer than 4 bytes.
