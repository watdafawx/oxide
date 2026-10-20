Last quests you wrote `Display`, `Error` and `From` by hand. The **thiserror** crate writes them from attributes. It is already in `Cargo.toml`.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("file {0} not found")]
    NotFound(String),

    #[error("bad number: {0}")]
    Parse(#[from] std::num::ParseIntError),
}
```

- **`#[derive(Error)]`** implements `std::error::Error` for you.
- **`#[error("...")]`** is the `Display` text. `{0}` is the first field, `{name}` a named one.
- **`#[from]`** on a field generates `From<ThatType>`, so `?` converts.

Same result as hand-written code, a quarter of the lines.

- **`ok_or_else(|| err)`** turns a `None` into `Err(err)`, building the error only when it is needed: `pairs.iter().find(..).ok_or_else(|| MyError::Missing(key.to_string()))?`.

## Your turn

- Open `src/lib.rs`. The variants have placeholder messages.
- Set the right `#[error]` text and add `#[from]` to `BadPort`.
- Write `read_port`: look up the key, then parse the value with `?`.
