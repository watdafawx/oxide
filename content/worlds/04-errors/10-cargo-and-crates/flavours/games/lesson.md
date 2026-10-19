**Why now:** your loader is about to need help: reading JSON manifests, nicer errors. Adding a **crate** to your project is exactly like adding a library mod to a modpack: you name it, pick a version range, and Cargo fetches it. `Cargo.toml` is your mod list.

```toml
[package]                  # who you are
name = "modpack_loader"
version = "0.1.0"
edition = "2024"

[dependencies]             # needed when the program runs
anyhow = "1"
serde = "1.2"              # means ^1.2: 1.2.0 up to, not including, 2.0.0

[dev-dependencies]         # only for tests and examples
tempfile = "3"
```

- **`[package]`** is metadata. **`[dependencies]`** ships with your program. **`[dev-dependencies]`** doesn't.
- **Caret versions** (`"1.2"` = `^1.2`): Cargo may take any compatible update. In semver, a new major version (2.0) may break your code, so it stays out. Same idea as a mod dropping support for old saves in its next major release.
- **`cargo add anyhow`** edits `Cargo.toml` for you. `cargo add --dev tempfile` adds a dev-dependency.
- **`Cargo.lock`** records the exact versions that were chosen, so builds repeat. It is your pinned modpack lockfile.

## Your turn

Answer the two questions below.
