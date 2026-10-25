To get the first word of a sentence you could copy those letters into a new `String`. But the letters are already there in memory; all you need is to point at part of them.

**A window into text.** `&text[0..4]` borrows the letters at positions 0, 1, 2 and 3: a **string slice**. It is a `&str` like any other, pointing into the original, so nothing is copied and nothing new goes on the heap.

**Ranges, short forms.** `&text[..4]` starts at the beginning, and `&text[5..]` runs to the end. These are the same `..` ranges as in the last world's loops.

**Find the cut.** `text.find(' ')` gives `Some(i)`, the position of the first space, or `None` when there is none. Slice up to `i` for the word before the space, or from `i + 1` for what comes after it.

```rust
let line = "jam 3 jars";
let item = match line.find(' ') {
    Some(i) => &line[..i],      // "jam"
    None => line,               // no space: the whole line
};
```

**Tied to the original.** A slice borrows, so the original must stay alive, and unchanged, while the slice is used. The borrow checker watches that, as with any `&`. One borrowed input and one borrowed output means Rust knows the slice came from `text`.

**For the curious.** Positions count bytes, not letters. Most letters are one byte, but `é` is two, and a slice that cuts through the middle of one panics. The next world shows how to walk letters safely.

## Your turn

- In `first_word`, give back everything before the first space, or all of `text` when there is none.
- In `after_first_word`, give back everything after the first space, or `""` when there is none.
