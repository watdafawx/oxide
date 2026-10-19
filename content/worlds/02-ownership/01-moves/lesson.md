Ownership is why Rust needs no garbage collector: the compiler always knows who frees each value.

**Every value has exactly one owner.** Assigning a `String` or passing it to a function **moves** it. The old name stops working.

```rust
let a = String::from("hi");
let b = a;            // the String moved from a to b
// println!("{a}");   // error: borrow of moved value: `a`
println!("{b}");      // fine
```

If you need two, ask for a second one on purpose: **`.clone()`** copies the text. Later quests show cheaper options (borrowing), but clone always works.

**This quest's starter does not compile.** That's the point: the compiler error is your task list. Fix it and the tests run.

## Your turn

- Open `src/lib.rs` and read the error under `length_and_name`.
- Fix `length_and_name` so the original String is still there at the end.
