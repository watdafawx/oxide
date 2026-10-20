**Why now:** your modpack manifest is a text file, and text lies. A line like `mods = 21x` must not crash the launcher. `?` lets you say "if this parse fails, hand the error back" in one character.

```rust
fn slots(count: &str) -> Result<i32, std::num::ParseIntError> {
    let n = count.parse::<i32>()?;   // Err? return it now. Ok? unwrap the value.
    Ok(n + 1)
}
```

- **`?`** on a `Result`: `Ok(v)` gives you `v`, `Err(e)` returns `Err(e)` from your function.
- On an `Option`: `Some(v)` gives `v`, `None` returns `None`.
- The function must return a matching type (`Result` or `Option`). `?` is an early **return**.

- **`c.to_uppercase()`** on a `char` returns an **iterator** (some letters become two), not a `char` or a `String`. Use `.collect::<String>()` to get a `String`, `.next()` for the first `char`, or `.chain(rest)` to glue it to the remaining chars first.

## Your turn

- Open `src/lib.rs`.
- In `mods_for_client_and_server`, parse the count with `?` and return `Ok(n * 2)`.
- In `mod_initial`, use `?` on `name.chars().next()` and return the uppercased letter (the launcher sorts mods by it).
