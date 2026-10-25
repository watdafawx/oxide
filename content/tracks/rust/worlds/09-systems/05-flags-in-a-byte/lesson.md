A status reply often packs several yes-or-no answers into one byte: is the server online, is it full, is it down for maintenance? Each answer is one bit. The checker needs to turn single bits on, off and test them without touching the others.

**Eight answers in a byte.** A byte has 8 bits, so it can hold 8 separate yes-or-no answers. Bit 0, the one worth 1, says "online"; bit 1, worth 2, says "full"; bit 2, worth 4, says "maintenance". `0b0000_0101` means online and in maintenance, not full.

**Names for bits.** `const ONLINE: u8 = 0b0000_0001;` gives a bit a name. A **const** is a value fixed when the program is built: it can never change, the type is required, and the name is written in CAPITALS by habit. A bit with a name is called a **flag**, and a value used to pick out bits is a **mask**.

**On with `|`.** `|` is **OR**: a bit is on in the result if it is on in either side. `status | FULL` turns the full bit on and leaves every other bit as it was.

**Test with `&`.** `&` is **AND**: a bit stays on only if it is on in both sides, so `status & FULL` keeps the full bit and clears the rest. If the result is not 0, the bit was on. For a mask of several bits, `status & mask == mask` asks "are all of them on?".

**Off with `& !`.** `!` flips every bit of a number, so `!FULL` is every bit except the full one. `status & !FULL` keeps everything but the full bit, which turns it off.

```rust
const BOLD: u8 = 0b0000_0001;
const ITALIC: u8 = 0b0000_0010;

let style = BOLD | ITALIC;            // 0b0000_0011, both on
assert!(style & ITALIC != 0);         // the italic bit is on
assert_eq!(style & !BOLD, ITALIC);    // bold turned off, italic kept
assert_eq!(style.count_ones(), 2);    // how many bits are on
```

**For the curious.** In C, `flags & MASK != 0` is a classic bug, because C reads it as `flags & (MASK != 0)`. Rust gives `&` a higher priority than `!=`, so it means what it looks like.

## Your turn

- Give `MAINTENANCE` the bit worth 4.
- Fill in `set_flag`, `has_flag` and `clear_flag`, one operator each.
- In `count_set_bits`, count the bits that are on with `count_ones()`.
