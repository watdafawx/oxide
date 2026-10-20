**Why now:** your `ManifestError` already needs `Display`, `Error` and `From<ParseIntError>`, and every new variant means more hand-written impls. The **thiserror** crate writes them from attributes. It is already in `Cargo.toml`.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum LaunchError {
    #[error("jar {0} not found")]
    NoJar(String),

    #[error("bad ram value: {0}")]
    Ram(#[from] std::num::ParseIntError),
}
```

- **`#[derive(Error)]`** implements `std::error::Error` for you.
- **`#[error("...")]`** is the `Display` text. `{0}` is the first field, `{name}` a named one.
- **`#[from]`** on a field generates `From<ThatType>`, so `?` converts.

Same result as hand-written code, a quarter of the lines.

- **`ok_or_else(|| err)`** turns a `None` into `Err(err)`, building the error only when it is needed: `pairs.iter().find(..).ok_or_else(|| MyError::Missing(key.to_string()))?`.
- **`find`** hands its closure a *reference* to each item. For `(name, score)` tuples, open them in the closure: `scores.iter().find(|(who, _)| *who == "ada")`. `who` is a `&&str`, so compare `*who`.

## Your turn

- Open `src/lib.rs`. The variants have placeholder messages.
- Set the right `#[error]` text and add `#[from]` to `BadVersion`.
- Write `find_version`: look up the mod by name in the manifest pairs, then parse its version with `?`.
