**Why now:** your `manifest` and `resolver` modules are getting long. Real mod loaders keep each one in its own file, so you can find the parser without scrolling past the dependency logic. `mod manifest;` in `lib.rs` means "load `src/manifest.rs`".

```rust
// src/lib.rs
pub mod worlds;          // reads src/worlds.rs

// src/worlds.rs
pub fn seed_for(name: &str) -> u64 { name.len() as u64 }

// src/other.rs
use crate::worlds::seed_for;   // crate:: = start at lib.rs
```

- **`mod name;`** (with a semicolon, no braces) pulls in `src/name.rs`.
- The file's contents *are* the module body. No extra `mod name { }` wrapper inside it.
- Other files reach siblings with **`crate::name::item`**.
- Items you want used elsewhere need **`pub`**.

## Your turn

- `src/lib.rs` already declares both modules.
- Open `src/manifest.rs` and write `parse_mod_line` for lines like `sodium@5`.
- Open `src/resolver.rs` and write `newest_mod` using `crate::manifest`.
