**Why now:** loading a manifest file can fail in unrelated ways: the file is missing (an `io::Error`), the format number is garbage (`ParseIntError`), the size is garbage (`ParseFloatError`), or the header has no comma. Writing one enum for all of it is overkill for a small tool.

```rust
use std::error::Error;

fn disk_needed(path: &str, extra: &str) -> Result<f64, Box<dyn Error>> {
    let text = std::fs::read_to_string(path)?;   // io::Error     -> Box
    let base: f64 = text.trim().parse()?;        // ParseFloatError -> Box
    let more: u32 = extra.parse()?;              // ParseIntError   -> Box
    if more > 50 {
        return Err("too many extras".into());    // &str -> Box
    }
    Ok(base + more as f64)
}
```

- **`Box<dyn Error>`** is "any error, on the heap". Anything implementing `Error` fits.
- `?` converts automatically. A string literal works with `.into()` or as the `?` source.
- Downside: callers can't `match` on the exact type. Fine for apps, weaker for libraries.

A pack header is one line: `format,size_gb`, such as `3,4.5` (pack format 3, 4.5 GB download).

## Your turn

- Open `src/lib.rs`.
- In `parse_pack_header`, split `"3,4.5"` at the comma and parse both halves. A missing comma must give the message `missing comma`.
- In `load_pack_header`, read the file and parse its text with that function.
