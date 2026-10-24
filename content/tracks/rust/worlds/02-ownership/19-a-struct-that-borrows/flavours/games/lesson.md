The console turns each typed line into a `Command` with a name and an argument. Copying both pieces into new Strings for every line is waste: the command could borrow them from the line. A struct that borrows has to say how long the borrowed text lives, and today you write that yourself.

**The compiler's offer.** Give `Command` a `name: &str` field and Rust stops, as it did last quest. This time, take the help it offers:

```text
error[E0106]: missing lifetime specifier
help: consider introducing a named lifetime parameter
  |
  | pub struct Command<'a> {
  |     pub name: &'a str,
```

**A name for "how long".** `'a` (say "tick a") is a **lifetime**: a name for the stretch of the program during which a borrow is valid. `struct Command<'a>` means "a Command holds borrows that live for some stretch `'a`", and `name: &'a str` says the name is one of them.

**A promise, checked.** A lifetime never makes anything live longer. It lets the compiler check that no `Command` is used after the line it borrows from is dropped. Use one too late and the build fails at that line.

```rust
struct Whisper<'a> {
    to: &'a str,
    text: &'a str,
}

fn reply_to(sender: &str) -> Whisper<'_> {
    Whisper { to: sender, text: "on my way" }
}
```

**Let Rust fill it in.** `'_` in `Whisper<'_>` means "the obvious lifetime": with one borrow coming in, `sender`, the struct borrows from it. A literal like `"on my way"` lives for the whole program, so it fits any `'a`.

**Owned or borrowed?** Own by default, as with `Player` in the last quest. Borrow when the value is short-lived and copying would cost: a parsed command lives only until it has run.

## Your turn

- Save and read the first error, on `Command`.
- Add `<'a>` after `Command` and make both fields `&'a str`.
- In `parse`, slice the line at the first space and build a `Command` from the two parts.
