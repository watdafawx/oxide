`/compare` takes two parsed commands and shows the longer argument, without copying it. Each `Command` borrows from its own typed line, so the answer points into one of two lines, but which one? Rust needs to know, so it can check that nobody uses the answer after its line is dropped.

**Try it.** The starter's `longer_arg` has no lifetimes. Rust stops and, this time, says exactly why:

```text
error[E0106]: missing lifetime specifier
  = help: this function's return type contains a borrowed value,
          but the signature does not say whether it is borrowed from `a` or `b`
```

**Why it asks.** With one borrowed input, the answer can only borrow from it, so Rust fills in the lifetime itself. That is what happened in `command_name` and `parse`. With two, the answer could come from either, and the signature has to say.

**Say "both".** `fn longer_arg<'a>(a: Command<'a>, b: Command<'a>) -> &'a str` declares a lifetime `'a` in the angle brackets after the name, then gives both commands and the result the same one. Read it as: "the answer is valid only while both lines are."

```rust
fn pick<'a>(first: &'a str, second: &'a str, take_first: bool) -> &'a str {
    if take_first { first } else { second }
}
```

**What it protects.** If the console drops one of the typed lines while still holding the answer, the build fails at that line, because the answer might point into the dropped text. Nothing is checked while the server runs; it is all decided before.

**For the curious.** The fill-in rules are called **lifetime elision**: each borrowed input gets its own lifetime; if there is exactly one, the output gets it too; in a method, the output borrows from `&self`. Only when these rules can't decide do you write `'a` yourself.

## Your turn

- Save and read the error on `longer_arg`.
- Add `<'a>` after the name, and write `Command<'a>` for both inputs and `&'a str` for the result.
