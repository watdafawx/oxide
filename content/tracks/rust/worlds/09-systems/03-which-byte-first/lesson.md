The checker will send numbers bigger than a byte: a `u32` length is four bytes. Writing it down means choosing which of the four goes first, and the server only understands one of the two choices.

**Bigger than a byte.** 300 does not fit in one byte, so as a `u32` it is four bytes: `00 00 01 2c` in hex (1 × 256 + 44 = 300). A `u16` like the port 8080 is two bytes, `1f 90`.

**Two orders.** **Big-endian** sends the big end first, the way you write numbers on paper: `00 00 01 2c`. **Little-endian** sends the small end first: `2c 01 00 00`. Same number, same four bytes, opposite order. The name for "which order" is **endianness**.

**Why both exist.** Chip designers picked different orders decades ago. Intel's chips, and most chips today, store numbers little-endian, because adding starts from the small end. The internet's standards of the 1970s and 80s picked big-endian, so it is also called **network byte order**. Both stuck, so every protocol has to say which one it uses.

**What goes wrong.** Read big-endian bytes as little-endian and nothing crashes: you just get a different number. A 300-byte reply turns into one of 738,263,040 bytes, and your program waits for data that never comes.

**The methods.** Every integer type knows both orders. `to_be_bytes()` and `to_le_bytes()` give an array of bytes; `u16::from_be_bytes(array)` and `u16::from_le_bytes(array)` turn one back into a number.

```rust
let port: u16 = 8080;
assert_eq!(port.to_be_bytes(), [0x1f, 0x90]);   // big end first
assert_eq!(port.to_le_bytes(), [0x90, 0x1f]);   // small end first
assert_eq!(u16::from_be_bytes([0x1f, 0x90]), 8080);
```

## Your turn

- In `to_le` and `to_be`, give back the four bytes of `n` in that order.
- In `from_le` and `from_be`, turn the four bytes back into a `u32`.
