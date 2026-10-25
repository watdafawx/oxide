Now the hash itself. A file is a list of bytes, `&[u8]`, and `fs::read(path)` gives you exactly that as a `Vec<u8>`. A hash turns any number of bytes into one `u64` that changes completely when even one byte changes. FNV-1a is one of the simplest good ones, and its recipe uses three things you have not met yet.

**Hex numbers.** Hash constants are written in **hexadecimal**, base 16: the digits are `0`-`9` and then `a`-`f` for 10 to 15. In Rust a hex number starts with `0x`: `0xff` is 15 × 16 + 15 = 255, the biggest `u8`. One hex digit is exactly 4 bits, so a `u64` is always 16 hex digits, which is why hashes are usually shown that way. `_` may split long numbers for reading: `0x0000_0100_0000_01b3`.

**XOR mixes in a byte.** `a ^ b`, **XOR** (exclusive or), compares two numbers bit by bit and sets a bit where exactly one of them has it. Written in binary, base 2, which Rust starts with `0b`: `0b1100 ^ 0b1010` is `0b0110`. `hash ^= x` is the short form of `hash = hash ^ x`. Both sides must be the same type, so widen each `u8` byte first with `u64::from(byte)`.

**A multiply that wraps.** The recipe then multiplies by a big prime. The result overflows a `u64` almost every time, and you know from the foundations world that a plain `*` panics on overflow. `hash.wrapping_mul(p)` keeps only the low 64 bits instead, like a car's odometer rolling over from 999999 to 000000. For a hash that is exactly what you want.

**The recipe.** Start at the offset value. For each byte: XOR the byte in, then wrapping-multiply by the prime. The two constants are in the starter's doc comment; they are the spec, so copy them, do not invent them.

```rust
let mut tag: u8 = 0x5a;
for byte in b"ok" {                      // b"..": the bytes of a text, a &[u8; 2]
    tag ^= *byte;                        // byte is a &u8
    tag = tag.wrapping_mul(3);
}
```

## Your turn

- In `fnv1a`, start a `mut` hash at `FNV_OFFSET`.
- For each byte, XOR it in (widened to `u64`), then multiply by `FNV_PRIME` with `wrapping_mul`.
- Return the hash.
