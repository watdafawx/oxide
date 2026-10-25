The todo list should tell the user why a line was skipped: `skipped: 'high' is not a number`. Your `Label` trait can do that, but `format!` has never heard of `Label`. It only knows the standard library's own promise, and that is the one to keep.

**The standard promise.** The standard library has its own trait for "how this looks to a person": **`Display`**. A type that keeps it works inside `{}` in `format!` and gets `.to_string()` for free. Your `Label` was a home-made version of the same idea.

**Its one method.** `Display` lists one method: `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`. The trait fixes that signature, so you copy it exactly. The starter already has it, with a `todo!()` inside.

**Where the text goes.** `f` is the **formatter**, the place the finished text is written into: the `String` that `format!` is building, or the screen. You do not return a `String`; you write into `f`. It is `&mut` because writing into it changes it.

**write! is format! into f.** `write!(f, "{n} is above 5")` takes the same `{}` as `format!` but writes into `f`. It hands back a `fmt::Result`, which is `Ok(())` when the writing worked, so it can be the last line of each `match` arm.

```rust
impl fmt::Display for DueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DueError::NoDate => write!(f, "no due date"),
            DueError::PastMonthEnd(day) => write!(f, "day {day} is past the end of the month"),
        }
    }
}
```

**The fmt path.** `fmt` is the standard library's formatting module. `use std::fmt;` at the top lets you write `fmt::Display` instead of `std::fmt::Display`; a later quest explains modules and `use` properly. `fmt::Result` lives there too, and it is not the `Result` you know: it only says "the writing worked or not".

**Free extras.** With `Display` in place, `format!("skipped: {err}")` just works. `{:?}` still prints the `Debug` view, which is for programmers: `PastMonthEnd(40)`.

## Your turn

- In `fmt`, `match self` and `write!` one message per variant.
- In `explain`, match on `parse_priority(text)`: `Ok(n)` gives `"priority <n>"`, and `Err(e)` gives `"skipped: <message>"`, with `{e}` inside a `format!`.
