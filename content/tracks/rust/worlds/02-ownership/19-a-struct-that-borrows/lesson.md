A summary screen shows a hundred notes. Copying every title just to draw it once is waste: the screen could borrow the titles from the notes. A struct that borrows has to say how long the borrowed text lives, and today you write that yourself.

**The compiler's offer.** Give `NoteView` a `title: &str` field and Rust stops, as it did last quest. This time, take the help it offers:

```text
error[E0106]: missing lifetime specifier
help: consider introducing a named lifetime parameter
  |
  | pub struct NoteView<'a> {
  |     pub title: &'a str,
```

**A name for "how long".** `'a` (say "tick a") is a **lifetime**: a name for the stretch of the program during which a borrow is valid. `struct NoteView<'a>` means "a NoteView holds a borrow that lives for some stretch `'a`", and `title: &'a str` says the title is that borrow.

**A promise, checked.** A lifetime never makes anything live longer. It lets the compiler check that no `NoteView` is used after the note it borrows from is dropped. Use one too late and the build fails at that line.

```rust
struct JarView<'a> {
    label: &'a str,
    grams: u32,
}

fn view(jar: &Jar) -> JarView<'_> {
    JarView { label: &jar.label, grams: jar.grams }
}
```

**Let Rust fill it in.** `'_` in `JarView<'_>` means "the obvious lifetime": with one borrow coming in, `&jar`, the view borrows from it. `&jar.label` is a `&String`, which Rust turns into the `&str` the field wants.

**Owned or borrowed?** Own by default, as in the last quest. Borrow when the view is short-lived and copying would cost: a screen, a parsed line, a search result.

## Your turn

- Save and read the first error, on `NoteView`.
- Add `<'a>` after `NoteView` and make the `title` field `&'a str`.
- In `view`, build a `NoteView` whose `title` borrows `&self.title`.
