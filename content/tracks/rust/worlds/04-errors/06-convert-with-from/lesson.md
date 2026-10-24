A todo file holds many lines, and the list wants the total priority of all of them. If one line is broken, the total is meaningless, so the first error should stop the sum. `?` does that in one character, but only once it knows how to turn the parser's error into yours.

**Refresher: `?`.** You met `?` in the collections world. Written after a `Result`, it gives you the value inside an `Ok` and carries on, or returns the `Err` from your function right there. It is the early `return Err(...)` from two quests ago, one character long.

**Two error types.** `text.parse::<u32>()` fails with the standard library's own error, **`ParseIntError`**, which lives at `std::num::ParseIntError`. Your function promises `TaskError`. On its own, `?` would return the wrong type, so the compiler refuses until you say how one becomes the other.

**The From promise.** The standard library has a trait for "a value of one type can become another": **`From`**. Its one method, `fn from(e: ParseIntError) -> Self`, takes the old error and builds yours. **`Self`** means "the type this impl is for", here `TaskError`.

**Angle brackets fill a blank.** `From<ParseIntError>` names the type it converts from, the same way `Vec<u32>` names what a `Vec` holds. So one error type can keep `From<ParseIntError>` and `From<ParseFloatError>` side by side: the same promise with a different blank.

```rust
impl From<ParseFloatError> for HoursError {
    fn from(e: ParseFloatError) -> Self {
        HoursError::NotHours(e)
    }
}
```

**? calls it for you.** Before `?` returns an error, it passes it through `From::from`. With the impl in place, `text.parse()?` inside a function that returns `TaskError` just works: the `ParseIntError` is wrapped on its way out.

**For the curious.** Keeping `From` also gives you **`Into`** for free, the same conversion seen from the other side: `let e: TaskError = parse_err.into();`. The standard library writes `into` for every `From` impl, which is why you only ever write `From`.

## Your turn

- In `from`, wrap the parse error in `TaskError::BadPriority`.
- In `total_priority`, add up ``parse_priority(line)?`` for every line with a `for` loop, and return `Ok(total)` at the end.
