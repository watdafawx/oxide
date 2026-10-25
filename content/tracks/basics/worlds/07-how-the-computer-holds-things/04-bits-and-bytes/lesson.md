Every colour, letter and number in the painter is, deep down, a row of on/off switches. Rust number types like `u8` and `u32`, file sizes and colour codes all make sense once you can read that row.

**A bit.** The smallest thing a computer stores is a **bit**: one switch, off or on, written 0 or 1. Everything else, from numbers to text to pictures, is built from many bits in a row.

**Counting in twos.** A row of bits is a number written in **binary**. Each place is worth twice the place to its right: 1, 2, 4, 8 and so on. So `101` is 4 + 0 + 1 = 5. People count in tens because they have ten fingers; computers count in twos because a switch has two positions.

**A byte.** Eight bits make a **byte**. With all 8 off the value is 0, and with all 8 on it is 255, so a byte can hold 256 different values. That is why a colour's red, green and blue each go from 0 to 255.

```text
bits:    1    1    0    0    1    0    0    0
worth: 128   64   32   16    8    4    2    1
value: 128 + 64 + 8 = 200
```

**Bigger numbers.** More bytes hold bigger numbers: two bytes reach 65,535, and four bytes about 4 billion. Rust names its number types by their size in bits, so `u8` is one byte (0 to 255) and `u32` is four bytes. Pick a type that is too small and the number does not fit.

**Sizes.** File and memory sizes count bytes: a kilobyte is about a thousand of them, a megabyte about a million and a gigabyte about a billion. A picture of 1000 by 1000 pixels with 3 bytes per pixel needs 3 million bytes, about 3 MB.

## Your turn

Answer the quiz, then start the next quest.
