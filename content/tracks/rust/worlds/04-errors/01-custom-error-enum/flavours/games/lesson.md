**Why now:** a manifest line can be broken three ways: no mod name, a version that is not a number, a loader nobody has heard of. The launcher must tell the player *which*, so one `String` error is not enough. You need a `ManifestError` enum.

```rust
use std::fmt;

#[derive(Debug)]
enum JarError { Missing(String), Locked }

impl fmt::Display for JarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JarError::Missing(name) => write!(f, "{name}.jar not found"),
            JarError::Locked => write!(f, "jar is locked"),
        }
    }
}

impl std::error::Error for JarError {}
```

- **`Display`** is the human message. **`Debug`** (derive it) is for programmers.
- **`impl std::error::Error`** marks your type as an error. It can be empty: `Debug` + `Display` are all it needs.
- With that, your type works with `?`, `Box<dyn Error>` and every library.

A manifest line looks like `sodium@5 fabric`: mod name, `@`, version number, then the loader. The valid loaders are `fabric`, `forge` and `neoforge` (the `Loader` enum); any other word is `UnknownLoader`.

- You can **match on text**: `match loader { Some("fabric") => ..., Some(other) => ..., None => ... }`. A literal like `"fabric"` is a pattern, and `other` catches everything else.

## Your turn

- Open `src/lib.rs`. `impl Error` is already there.
- Write `Display for ManifestError`: one `write!` per variant.
- Fill in `parse_mod_line` so each broken line gives the right variant.
