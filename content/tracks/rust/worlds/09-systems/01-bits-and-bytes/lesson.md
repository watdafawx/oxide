This world builds a status checker: a program that asks a server "are you up, and how busy are you?" and prints the answer. Servers don't answer in words. They answer in **bytes**, and before you can read a single reply you need to see what a byte is.

**A bit.** The smallest thing a computer stores is a **bit**: one switch, off or on, written 0 or 1. Text, numbers and pictures are all long rows of bits.

**Counting in twos.** A row of bits is a number in **binary**. Each place is worth twice the place to its right: 1, 2, 4, 8, 16 and so on. So `101` is 4 + 0 + 1 = 5. Computers count in twos because a switch has two positions.

**A byte.** Eight bits make a **byte**. All eight off is 0, all eight on is 128 + 64 + 32 + 16 + 8 + 4 + 2 + 1 = 255. That is exactly Rust's `u8`: one byte, 0 to 255. A `u16` is two bytes and a `u32` is four; the number in the name counts bits.

```text
bits:    1    1    0    0    1    0    0    0
worth: 128   64   32   16    8    4    2    1
value: 128 + 64 + 8 = 200
```

**Binary in Rust.** Put `0b` in front of a number and Rust reads it as binary: `0b101` is 5, `0b1100_1000` is 200. The `_` is a spacer that Rust ignores, so you can group the bits in fours; `25_565` works the same way for normal numbers.

**Why it matters here.** A network reply is a list of `u8`s, a `&[u8]`. Every field in it, a version, a port, a length, is one or more of those bytes, and the next quests take them apart.

**If this is new.** Programming 101 explains bits and bytes from zero, with pictures: [Bits and bytes](/quest/basics/07-how-the-computer-holds-things/04-bits-and-bytes).

## Your turn

Answer the questions, then start the next quest.
