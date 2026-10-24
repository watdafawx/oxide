The loader's code has outgrown one file. The manifest reader and the resolver each get their own file, the way real mod loaders keep them, so you can find the parser without scrolling past the dependency logic. The files are already in `src/`, but Rust does not know about them yet.

**A file is a room.** `mod manifest;` with a semicolon, instead of braces, means "this room's contents are in `src/manifest.rs`". The file holds the body directly: no `mod manifest { }` wrapper inside it.

**Only named files count.** Cargo compiles `lib.rs` and the files it names with `mod`, nothing else. `src/manifest.rs` sitting in the folder does nothing until then, which is why the starter does not build. Read the help line:

```text
error[E0433]: cannot find module or crate `manifest` in this scope
help: to make use of source file src/manifest.rs, use `mod manifest` in this file to declare the module
```

**pub mod.** `pub mod manifest;` also opens the room to code outside the crate, the way `pub` opened a function last time.

**Siblings use crate::.** `resolver.rs` is its own room, next to `manifest`, not inside it. To reach its sibling it starts from the top: `crate::manifest::version_of`.

```rust
// src/lib.rs
pub mod clock;                 // the body is in src/clock.rs

// src/clock.rs
pub fn minutes(hours: u32) -> u32 {
    hours * 60
}

// src/calendar.rs, a sibling room
use crate::clock::minutes;
```

## Your turn

- In `lib.rs`, declare both files as public modules: `manifest` and `resolver`.
- In `resolver.rs`, write `pack_fingerprint`: add up `crate::manifest::version_of` of every mod line.
