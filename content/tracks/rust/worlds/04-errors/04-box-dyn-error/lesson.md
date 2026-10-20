Some functions hit several error types: a parse int, a parse float, a custom message. Writing an enum for each is overkill in small programs.

```rust
use std::error::Error;

fn total(a: &str, b: &str) -> Result<f64, Box<dyn Error>> {
    let x: i32 = a.parse()?;       // ParseIntError  -> Box
    let y: f64 = b.parse()?;       // ParseFloatError -> Box
    if y < 0.0 {
        return Err("negative".into());   // &str -> Box
    }
    Ok(x as f64 + y)
}
```

- **`Box<dyn Error>`** is "any error, on the heap". Anything implementing `Error` fits.
- `?` converts automatically. A string literal works with `.into()` or as the `?` source.
- Downside: callers can't `match` on the exact type. Fine for apps, weaker for libraries.

- Text from a file has stray spaces and a trailing `\n`, and `parse` rejects both. **`trim()`** each part first: `" 10 ".trim().parse::<u32>()`.

## Your turn

- Open `src/lib.rs`.
- In `parse_pair`, split `"3,4.5"` at the comma and parse both halves.
- A missing comma must produce the message `missing comma`.
- Trim each half before you parse it: `" 3 , 4.5 "` is valid input.
