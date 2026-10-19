**Why now:** the mod database API works, and the next quest draws it in egui. A screen is mostly **text built from data**, so build that text first as plain functions: easy to write, trivial to test, no window needed.

```rust
let mark = if enabled { "[x]" } else { "[ ]" };   // `if` is an expression
let line = format!("{mark} {name}");               // "[x] sodium"

let on = mods.iter().filter(|(_, enabled)| *enabled).count();
let summary = format!("{on} of {} mods enabled", mods.len());
```

- `if ... { a } else { b }` returns a value: no temp variable, no ternary operator.
- `format!` builds a `String` from pieces. `{name}` inlines a variable, `{}` takes the next argument.
- `iter().filter(..).count()` counts what matches. A tuple pattern in the closure (`|(_, enabled)|`) picks the field you want.
- This is the **view model** idea: the screen asks a function for its text, and the test checks the function.

## Your turn

In `src/lib.rs`:

- `card_line`: `"[x] sodium"` when enabled, `"[ ] sodium"` when not.
- `pack_summary`: `"2 of 3 mods enabled"` for a slice of `(name, enabled)` pairs.
