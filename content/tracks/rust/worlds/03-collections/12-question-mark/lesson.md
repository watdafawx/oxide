Real programs fail all the time: bad input, missing files. `?` makes handling that one character long.

```rust
fn double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = s.parse::<i32>()?;   // Err? return it now. Ok? unwrap the value.
    Ok(n * 2)
}
```

- **`?`** on a `Result`: `Ok(v)` gives you `v`, `Err(e)` returns `Err(e)` from your function.
- On an `Option`: `Some(v)` gives `v`, `None` returns `None`.
- The function must return a matching type (`Result` or `Option`). `?` is an early **return**.

- **`c.to_uppercase()`** on a `char` returns an **iterator** (some letters become two), not a `char` or a `String`. Use `.collect::<String>()` to get a `String`, `.next()` for the first `char`, or `.chain(rest)` to glue it to the remaining chars first.

## Your turn

- Open `src/lib.rs`.
- In `parse_and_double`, parse with `?` and return `Ok(n * 2)`.
- In `first_char_upper`, use `?` on `s.chars().next()` and return the uppercased char.
