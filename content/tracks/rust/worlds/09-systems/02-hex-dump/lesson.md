A status checker spends a lot of time looking at bytes it got back. Written in binary, a four-byte reply is 32 zeros and ones, and nobody can read that. Programmers write bytes in hexadecimal instead, and your first tool is a function that prints a reply that way.

**Counting in sixteens.** **Hexadecimal**, hex for short, counts in sixteens. After the digits 0 to 9 come the letters a to f, worth 10 to 15. So `f` is 15, and `10` in hex is one sixteen and no ones: 16.

**Two digits, one byte.** One hex digit holds exactly 4 bits (0 to 15), so two hex digits hold one byte. `00` is 0, `ff` is 15 × 16 + 15 = 255, `10` is 16. Every byte is exactly two hex digits, which is why protocol documents write replies like `10 00 ff 05`.

**Hex in Rust.** A `0x` in front makes a number hex: `0xff` is simply 255 written another way, and `0x1F90` is 8080. With the spacer, `0x1234_5678` shows its four bytes at a glance: `12`, `34`, `56`, `78`.

**Printing hex.** In `format!`, `{:x}` prints a number in hex, as for the file hashes in world 07. Add `02` in front, `{:02x}`, and it always prints at least two digits, padding with a 0. Without it, 5 prints as `5` and a dump of `[0x10, 0x05]` reads `10 5`, which is ambiguous.

```rust
let port: u16 = 8080;
println!("{:x}", port);      // 1f90
println!("{:02x}", 7u8);     // 07
println!("{:02X}", 171u8);   // AB  (a capital X gives capital letters)
```

**If this is new.** Programming 101 has a whole quest on hex, with colour codes: [Hex colours](/quest/basics/07-how-the-computer-holds-things/05-hex-colours).

## Your turn

- In `hex_dump`, turn every byte into two lowercase hex digits.
- Put one space between the pairs, and nothing at the ends.
