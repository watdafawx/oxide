**Why now:** `parse_build` has two ways to fail: the text is not a number (a `ParseIntError`), or the build is newer than your launcher knows (`TooNew`). `?` only passes along the *same* error type, so you teach it the conversion with **`From`**.

```rust
enum LaunchError { BadRam(std::num::ParseIntError) }

impl From<std::num::ParseIntError> for LaunchError {
    fn from(e: std::num::ParseIntError) -> Self {
        LaunchError::BadRam(e)
    }
}

fn ram_gb(s: &str) -> Result<u32, LaunchError> {
    let n: u32 = s.parse()?;   // ParseIntError -> LaunchError, automatically
    Ok(n)
}
```

- `impl From<A> for B` says "an `A` can become a `B`".
- `?` runs `From::from` on the error for you.
- You get `Into` for free: `let b: B = a.into();`.

## Your turn

- Open `src/lib.rs`.
- Fill in the body of `from` so a `ParseIntError` becomes `ManifestError::BadVersion`.
- Write `pack_fingerprint` (the sum of every build number in the pack) using `parse_build(build)?` in a loop.
