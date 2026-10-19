`?` only returns the *same* error type. When the types differ, it calls **`From`** to convert.

```rust
enum AppError { Parse(std::num::ParseIntError) }

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn read(s: &str) -> Result<i32, AppError> {
    let n: i32 = s.parse()?;   // ParseIntError -> AppError, automatically
    Ok(n)
}
```

- `impl From<A> for B` says "an `A` can become a `B`".
- `?` runs `From::from` on the error for you.
- You get `Into` for free: `let b: B = a.into();`.

## Your turn

- Open `src/lib.rs`.
- Fill in the body of `from` so a `ParseIntError` becomes `MyError::Parse`.
- Write `sum_all` using `parse_small(input)?` in a loop.
