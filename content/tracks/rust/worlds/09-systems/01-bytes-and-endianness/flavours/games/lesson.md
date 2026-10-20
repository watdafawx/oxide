**Why now:** the server pinger sends and receives raw bytes, and the first number in a reply (the protocol version, `763`) is more than one byte long. Which byte comes first? That is **endianness**, and mixing it up gives you a version of 1,000,013,824 instead of 763.

```rust
let protocol: u32 = 763;                 // 0x0000_02FB

let le = protocol.to_le_bytes();         // [0xFB, 0x02, 0x00, 0x00] small end first
let be = protocol.to_be_bytes();         // [0x00, 0x00, 0x02, 0xFB] big end first (network order)

let back = u32::from_be_bytes(be);       // 763
```

- **Little-endian** (x86, ARM): least significant byte first. **Big-endian**: most significant first. Network protocols are usually big-endian ("network byte order").
- `to_le_bytes`, `to_be_bytes`, `from_le_bytes`, `from_be_bytes` exist on every integer type.
- `from_*_bytes` wants an **array** `[u8; 4]`, not a slice. `slice.try_into()` converts when the length is exactly 4.
- In a function that returns `Option`, chain the steps: `let first: [u8; 4] = reply.get(..4)?.try_into().ok()?;` (`get(..4)?` leaves early with `None` when short; `try_into()` is a `Result`, so `.ok()?` turns its `Err` into the same early `None`).
- Reading from a network reply: never index blindly. `bytes.get(..4)` returns `None` instead of panicking when the reply is short.

## Your turn

In `src/lib.rs`:

- `protocol_to_le` / `protocol_to_be`: a `u32` as 4 bytes.
- `read_protocol_le` / `read_protocol_be`: the first 4 bytes of a slice as a `u32`; `None` when fewer than 4 bytes.
