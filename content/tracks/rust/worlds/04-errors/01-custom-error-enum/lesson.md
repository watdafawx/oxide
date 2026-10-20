A plain `String` error can't be matched on. An **enum** can: callers pick what to do per case.

```rust
use std::fmt;

#[derive(Debug)]
enum FileError { Missing(String), Locked }

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::Missing(name) => write!(f, "{name} not found"),
            FileError::Locked => write!(f, "file is locked"),
        }
    }
}

impl std::error::Error for FileError {}
```

- **`Display`** is the human message. **`Debug`** (derive it) is for programmers.
- **`impl std::error::Error`** marks your type as an error. It can be empty: `Debug` + `Display` are all it needs.
- With that, your type works with `?`, `Box<dyn Error>` and every library.

## Your turn

- Open `src/lib.rs`. `impl Error` is already there.
- Write `Display for ParseError`: one `write!` per variant.
- Fill in `parse_age` so each bad input gives the right variant.
