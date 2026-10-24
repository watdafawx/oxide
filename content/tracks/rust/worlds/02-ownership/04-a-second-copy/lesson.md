Sometimes two parts of a program both want to own the same text. A function that takes a `String` keeps it for good, but you still need yours after the call.

**Calling moves too.** A parameter typed `String` takes ownership: `name_length(name)` moves `name` into the function, and the function drops it at its own `}`. Your `name` is gone after the call.

**Ask for a second one.** `.clone()` makes a brand-new `String`: new room on the heap, every letter copied. Now there are two values with two owners, and each is dropped on its own.

```rust
let label = String::from("crate 7");
let spare = label.clone();     // a second String with its own letters
shelve(spare);                 // spare moves into shelve
let shown = label.len();       // label is still ours
```

**Clone costs.** Copying letters takes time and memory, so Rust never clones behind your back: you see every `.clone()` you write. For a short name it does not matter. The next quests show how to lend a value instead, with no copy at all.

**Clone is not Copy.** A number is copied silently because four bytes are the whole value. A `String` is never copied silently, because a real copy means asking the heap for new room.

## Your turn

- Save and read the error under `length_and_name`.
- Give `name_length` a clone of `name`, and keep the original for the tuple.
