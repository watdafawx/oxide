Every Minecraft release has a protocol version number, and 1.20.1 is `763`. That does not fit in one byte, so as a `u32` it is four bytes. Sending it means choosing which of the four goes first, and the server only understands one of the two choices.

**Bigger than a byte.** 763 is 2 × 256 + 251, so as a `u32` it is `00 00 02 fb` in hex. A `u16` like the port 25565 is two bytes, `63 dd`.

**Two orders.** **Big-endian** sends the big end first, the way you write numbers on paper: `00 00 02 fb`. **Little-endian** sends the small end first: `fb 02 00 00`. Same number, same four bytes, opposite order. The name for "which order" is **endianness**.

**Why both exist.** Chip designers picked different orders decades ago. Intel's chips, and most chips today, store numbers little-endian, because adding starts from the small end. The internet's standards of the 1970s and 80s picked big-endian, so it is also called **network byte order**. Both stuck, so every protocol has to say which one it uses.

**What goes wrong.** Read big-endian bytes as little-endian and nothing crashes: you just get a different number. Version 763 turns into version 4,211,212,288, and the launcher says the server runs a Minecraft from the far future.

**The methods.** Every integer type knows both orders. `to_be_bytes()` and `to_le_bytes()` give an array of bytes; `u16::from_be_bytes(array)` and `u16::from_le_bytes(array)` turn one back into a number.

```rust
let port: u16 = 25565;
assert_eq!(port.to_be_bytes(), [0x63, 0xdd]);   // big end first
assert_eq!(port.to_le_bytes(), [0xdd, 0x63]);   // small end first
assert_eq!(u16::from_be_bytes([0x63, 0xdd]), 25565);
```

## Your turn

- In `protocol_to_le` and `protocol_to_be`, give back the four bytes of the version in that order.
- In `protocol_from_le` and `protocol_from_be`, turn the four bytes back into a `u32`.
