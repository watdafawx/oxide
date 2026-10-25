The stock tool should write a log line every time the shelf changes, so you can see what happened when something looks wrong. A whole list has to go into that line, and `format!` does not know how to show one yet.

**Try the obvious thing.** `format!("stock: {}", counts)` works for one number, so try it on the list. Rust refuses to build:

```text
error[E0277]: `[u32]` doesn't implement `std::fmt::Display`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

**What the red means.** **Display** is the skill "knows how to show itself to a user". Numbers and text have it. A list does not, because Rust cannot guess what a user wants: commas, one per line, a table? Rust calls a skill like this a **trait**: something a type can do, or not. A later world shows how to give your own types one.

**The programmer's view.** The note already has the fix. `{:?}` prints with **Debug**, a second trait meant for programmers: plain and unfussy, with square brackets, commas and quotes around text. Lists, `Option`s and most of Rust's own types have it.

```rust
format!("{:?}", vec![150, 400])    // "[150, 400]"
format!("{:?}", Some(3))           // "Some(3)"
```

**You have seen it already.** When an `assert_eq!` fails, the left and right values you read are Debug prints. That is why a test can show you a whole list that came out wrong.

## Your turn

- In `stock_log`, try `{}` first and read the error. Then switch to `{:?}`.
- `shelf_log` is the same, for names. Look at the test: Debug puts quotes around text.
