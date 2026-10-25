The log keeps every chat line you hand it, and the console still wants to broadcast the same line to everyone online. Two parts of the program both want to own the same text.

**Calling moves too.** A parameter typed `String` takes ownership: `log_length(line)` moves `line` into the function, and the function drops it at its own `}`. Your `line` is gone after the call.

**Ask for a second one.** `.clone()` makes a brand-new `String`: new room on the heap, every letter copied. Now there are two values with two owners, and each is dropped on its own.

```rust
let motd = String::from("Welcome to the server");
let saved = motd.clone();      // a second String with its own letters
save_motd(saved);              // saved moves into save_motd
let shown = motd.len();        // motd is still ours
```

**Clone costs.** Copying letters takes time and memory, so Rust never clones behind your back: you see every `.clone()` you write. For one chat line it does not matter. The next quests show how to lend a value instead, with no copy at all.

**Clone is not Copy.** A player's level is copied silently because four bytes are the whole value. A `String` is never copied silently, because a real copy means asking the heap for new room.

## Your turn

- Save and read the error under `length_and_line`.
- Give `log_length` a clone of `line`, and keep the original for the tuple.
