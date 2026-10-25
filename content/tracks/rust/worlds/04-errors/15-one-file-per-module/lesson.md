The todo list's code has outgrown one file. The parser and the report each get their own file, so you can find the parsing code without scrolling past the printing code. The files are already in `src/`, but Rust does not know about them yet.

**A file is a room.** `mod parser;` with a semicolon, instead of braces, means "this room's contents are in `src/parser.rs`". The file holds the body directly: no `mod parser { }` wrapper inside it.

**Only named files count.** Cargo compiles `lib.rs` and the files it names with `mod`, nothing else. `src/parser.rs` sitting in the folder does nothing until then, which is why the starter does not build. Read the help line:

```text
error[E0433]: cannot find module or crate `parser` in this scope
help: to make use of source file src/parser.rs, use `mod parser` in this file to declare the module
```

**pub mod.** `pub mod parser;` also opens the room to code outside the crate, the way `pub` opened a function last time.

**Siblings use crate::.** `report.rs` is its own room, next to `parser`, not inside it. To reach its sibling it starts from the top: `crate::parser::priority_of`.

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

- In `lib.rs`, declare both files as public modules: `parser` and `report`.
- In `report.rs`, write `total_priority`: add up `crate::parser::priority_of` of every line.
