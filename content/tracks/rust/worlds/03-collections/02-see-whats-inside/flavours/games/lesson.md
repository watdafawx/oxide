Your plugin should write a debug line every time the hotbar changes, so you can see what happened when an item goes missing. A whole list has to go into that line, and `format!` does not know how to show one yet.

**Try the obvious thing.** `format!("hotbar: {}", stacks)` works for one number, so try it on the list. Rust refuses to build:

```text
error[E0277]: `[u32]` doesn't implement `std::fmt::Display`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

**What the red means.** **Display** is the skill "knows how to show itself to a player". Numbers and text have it. A list does not, because Rust cannot guess what a player wants: commas, one per line, a grid? Rust calls a skill like this a **trait**: something a type can do, or not. A later world shows how to give your own types one.

**The programmer's view.** The note already has the fix. `{:?}` prints with **Debug**, a second trait meant for programmers: plain and unfussy, with square brackets, commas and quotes around text. Lists, `Option`s and most of Rust's own types have it.

```rust
format!("{:?}", vec![64, 32])    // "[64, 32]"
format!("{:?}", Some(3))         // "Some(3)"
```

**You have seen it already.** When an `assert_eq!` fails, the left and right values you read are Debug prints. That is why a test can show you a whole hotbar that came out wrong.

## Your turn

- In `hotbar_log`, try `{}` first and read the error. Then switch to `{:?}`.
- `item_log` is the same, for item names. Look at the test: Debug puts quotes around text.
