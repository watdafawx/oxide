A new version loader returns a `Box<dyn Error>`, like the pack-header reader from the last quest, and also checks the version with your own `ManifestError`. Your error has a variant and a message, but the box will not take it yet.

**The compiler stops you.** The starter does not build. `load_version` uses `?` on a `ManifestError` inside a function that returns `Box<dyn Error>`, and the compiler explains why that fails:

```text
error[E0277]: `?` couldn't convert the error: `ManifestError: std::error::Error` is not satisfied
note: `ManifestError` needs to implement `std::error::Error`
```

**Read the note.** A `Box<dyn Error>` only takes types that keep the `Error` promise. The standard errors keep it; yours never made it, so `?` has no way to put it in the box.

**An empty promise.** A trait can come with method bodies already written, called **default methods**, which a type gets unless it writes its own. Every method of `Error` has one, so keeping the promise can be a block with nothing inside:

```rust
impl std::error::Error for LoaderError {}
```

**What it asks first.** `Error` demands two other promises before it: `Debug`, printing for programmers, and `Display`, printing for people. Your `#[derive(Debug)]` and your `Display` impl cover both. That is why the last quests came first.

**Why mark it.** With the promise kept, your error goes anywhere an error is expected: into a `Box<dyn Error>`, into the error crates two quests from now, and into every library that takes "any error".

**For the curious.** One default method, `source()`, can return the error underneath this one, so a chain like "line 3 is broken, because: invalid digit" can be walked from top to bottom. The anyhow quests use that chain.

## Your turn

- Below the `Display` impl, keep the `Error` promise for `ManifestError` with an empty `impl` block.
- Save. The build error goes away and both objectives turn green.
