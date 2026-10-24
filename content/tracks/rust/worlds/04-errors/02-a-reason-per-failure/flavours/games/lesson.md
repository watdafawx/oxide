Each mod in the manifest has a version number, and this launcher knows versions up to 20. A typed version can be wrong in three ways: nothing after the `@`, a word like `new`, or a version newer than the launcher knows. With a `String` error, the launcher gets a sentence and has to guess which of the three happened.

**A sentence hides the kind.** `Err("'new' is not a number".to_string())` reads well, but code cannot ask it "was the version missing?". Comparing the text would break the day someone rewords the message.

**One variant per failure.** An error can be an `enum`, like any other choice between a few things. Each variant names one way to fail: `MissingVersion`, `BadVersion`, `TooNew`. The function returns `Result<u32, ManifestError>`: a version, or one of those.

**Carry the details.** A variant can hold data, as in the foundations world. `BadVersion(String)` keeps the text the player typed and `TooNew(u32)` keeps the number, so the message can show them later.

**Leave early.** `return Err(...)` ends the function right there, inside an `if` or inside a `match` arm. The checks then read top to bottom, and the last line is the `Ok` that only good input reaches.

```rust
enum LoaderError {
    NoName,
    Unknown(String),
}

fn check_loader(name: &str) -> Result<String, LoaderError> {
    if name == "" {
        return Err(LoaderError::NoName);
    }
    if name != "fabric" {
        return Err(LoaderError::Unknown(name.to_string()));
    }
    Ok(name.to_string())
}
```

**The launcher picks.** Now the launcher can `match` the error: skip a mod with `MissingVersion`, but tell the player to update on `TooNew(v)`. That choice is why the error is an enum and not a sentence.

## Your turn

- In `parse_version`, trim the text, then return `Err(ManifestError::MissingVersion)` when nothing is left.
- Parse it with `parse::<u32>()`; on `Err(_)` return `BadVersion` carrying the trimmed text.
- A version above `NEWEST` is `TooNew(v)`. Anything else is `Ok(v)`.
