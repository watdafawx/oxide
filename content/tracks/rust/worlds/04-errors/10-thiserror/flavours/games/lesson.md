Your `ManifestError` has grown three hand-written impls: `Display`, `Error` and `From<ParseIntError>`. Every new variant means editing all of them, like patching three config files for one mod. The **thiserror** crate writes them from a few notes on the enum, and it is already in this quest's `Cargo.toml`.

**Notes on your code.** `#[...]` written above an item is an **attribute**, a note for the compiler or for a crate. You have seen `#[derive(Debug)]` and the `#[test]` above every test. thiserror reads a few notes of its own.

**derive(Error).** With `use thiserror::Error;` at the top, `#[derive(Error)]` on the enum writes the `impl std::error::Error` block for you, the empty one from the last quest.

**#[error] is Display.** `#[error("...")]` above a variant is that variant's `Display` message. `{0}` inside it is the variant's first field, so `#[error("version {0} is too new")]` prints `version 99 is too new`.

**#[from] is From.** `#[from]` in front of a variant's field writes `impl From<ThatType>` for you, so `?` converts that error into this variant, as your hand-written `From` did.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum LoaderError {
    #[error("no loader given")]
    NoName,
    #[error("size is not a number: {0}")]
    BadSize(#[from] std::num::ParseFloatError),
}
```

**Why a crate for this.** The code thiserror writes is the code you wrote by hand, in a quarter of the lines. Callers can still `match` on every variant, which is why libraries, code other people call, use it for their errors.

## Your turn

- Give `MissingVersion` the message `no version` and `BadVersion` the message `version is not a number: {0}`.
- Mark `BadVersion`'s field with `#[from]`.
- In `parse_version`: trim, return `MissingVersion` for empty text, parse with `?`, and return `TooNew` for a number above `NEWEST`.
