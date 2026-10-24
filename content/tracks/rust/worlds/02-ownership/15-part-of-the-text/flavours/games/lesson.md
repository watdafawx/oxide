Every console line starts with a command word (`kick`, `tp`, `give`) followed by its arguments. You could copy each part into a new `String`, but the letters are already there in the line; all you need is to point at part of them.

**A window into text.** `&line[0..4]` borrows the letters at positions 0, 1, 2 and 3: a **string slice**. It is a `&str` like any other, pointing into the original, so nothing is copied and nothing new goes on the heap.

**Ranges, short forms.** `&line[..4]` starts at the beginning, and `&line[5..]` runs to the end. These are the same `..` ranges as in the crafting calculator's loops.

**Find the cut.** `line.find(' ')` gives `Some(i)`, the position of the first space, or `None` when there is none. Slice up to `i` for the word before the space, or from `i + 1` for what comes after it.

```rust
let typed = "msg Alex see you at spawn";
let cmd = match typed.find(' ') {
    Some(i) => &typed[..i],     // "msg"
    None => typed,              // no space: the whole line
};
```

**Tied to the original.** A slice borrows, so the line must stay alive, and unchanged, while the slice is used. The borrow checker watches that, as with any `&`. One borrowed input and one borrowed output means Rust knows the slice came from `line`.

**For the curious.** Positions count bytes, not letters. Most letters are one byte, but `é` in a player's chat is two, and a slice that cuts through the middle of one panics. The next world shows how to walk letters safely.

## Your turn

- In `command_name`, give back everything before the first space, or all of `line` when there is none.
- In `command_args`, give back everything after the first space, or `""` when there is none.
