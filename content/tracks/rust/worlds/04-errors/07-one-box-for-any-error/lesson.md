Each task line can carry an estimate: `3,1.5` is priority 3 and one and a half hours. Reading it can fail three ways: the priority is not a whole number, the hours are not a number, or the comma is missing. That is three different error types for one small function, and writing an enum plus two `From` impls for it feels like a lot.

**Why not "any error"?** Rust has to know how many bytes a function's return value takes before the program runs. A `ParseIntError`, a `ParseFloatError` and a sentence all have different sizes, so "one of several types" has no fixed size, and the compiler cannot use it as a return type.

**Box: a handle of fixed size.** `Box<T>` puts the value in a separate area of memory, the **heap**, which means memory the program asks for while it runs. The box itself is only a pointer to it, an address. An address has the same size whatever it points at, so the return type has a fixed size again.

**dyn: decided while running.** **`dyn Error`** means "some type that keeps the `Error` promise; which one is decided while the program runs". `Error` is the standard library's trait for error types, at `std::error::Error`. Put together, `Box<dyn Error>` reads "a pointer to any error".

**? still converts.** The standard library has `From` impls that turn every standard error into a `Box<dyn Error>`, and a plain `&str` sentence too. So `?` works on both parses, and `.into()` asks for that conversion by hand: `return Err("missing comma".into());`.

```rust
use std::error::Error;

fn parse_size(text: &str) -> Result<(u32, f64), Box<dyn Error>> {
    let (count, megabytes) = match text.split_once('x') {
        Some(pair) => pair,
        None => return Err("expected COUNTxMB".into()),
    };
    let count: u32 = count.trim().parse()?;      // a ParseIntError goes in the box
    let megabytes: f64 = megabytes.trim().parse()?; // so does a ParseFloatError
    Ok((count, megabytes))
}
```

**The price.** The caller can no longer `match` on which error it was; it can only print it. That is fine for a small tool or for `main`, and weaker for a library other people call, which is why the custom enum of the last quests still has its place.

**For the curious.** `fn main() -> Result<(), Box<dyn Error>>` lets `main` itself use `?`. Errors sent between threads need `Box<dyn Error + Send + Sync>`, a promise that the error is safe to move to another thread.

## Your turn

- In `parse_estimate`, cut the text at the comma with `split_once(',')`. On `None`, return `Err("missing comma".into())`.
- Parse the left half as a `u32` and the right half as an `f64`, trimming each and using `?`.
- Return both as `Ok((priority, hours))`.
