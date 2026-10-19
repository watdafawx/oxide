**Why now:** the console keeps a log of chat. When you hand a chat line to the log, the log owns it, and you can't print or broadcast that same line afterwards. That is a **move**, and it is why Rust needs no garbage collector: the compiler always knows who frees each value.

**Every value has exactly one owner.** Assigning a `String` or passing it to a function **moves** it. The old name stops working.

```rust
let line = String::from("<Ferris> hello");
let logged = line;         // the String moved from line to logged
// println!("{line}");     // error: borrow of moved value: `line`
println!("{logged}");      // fine
```

If you need two, ask for a second one on purpose: **`.clone()`** copies the text. Later quests show cheaper options (borrowing), but clone always works.

**This quest's starter does not compile.** That's the point: the compiler error is your task list. Fix it and the tests run.

## Your turn

- Open `src/lib.rs` and read the error under `length_and_name`.
- Fix `length_and_name` so the original chat line is still there at the end.
