When a module grows, give it its own file. `mod parser;` in `lib.rs` means "load `src/parser.rs`".

```rust
// src/lib.rs
pub mod shapes;          // reads src/shapes.rs

// src/shapes.rs
pub fn square(x: i32) -> i32 { x * x }

// src/other.rs
use crate::shapes::square;   // crate:: = start at lib.rs
```

- **`mod name;`** (with a semicolon, no braces) pulls in `src/name.rs`.
- The file's contents *are* the module body. No extra `mod name { }` wrapper inside it.
- Other files reach siblings with **`crate::name::item`**.
- Items you want used elsewhere need **`pub`**.

## Your turn

- `src/lib.rs` already declares both modules.
- Open `src/parser.rs` and write `parse_line` for lines like `ana: 10`.
- Open `src/report.rs` and write `best_player` using `crate::parser`.
