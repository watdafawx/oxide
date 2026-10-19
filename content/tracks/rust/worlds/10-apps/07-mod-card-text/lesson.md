**Why now:** the mod database and its API are done; the next two quests draw them on screen. Screens are easiest to test when the text comes from plain functions, so write those first.

You need no new crate or concept, just `format!` and iterators from earlier worlds:

```rust
fn tag(name: &str, count: usize) -> String {
    format!("{name} x{count}")      // "sodium x3"
}

let enabled = [true, false, true];
let on = enabled.iter().filter(|e| **e).count();   // 2
```

Oxide's own UI uses the same `[x] label` / `[ ] label` style for objectives.

## Your turn

In `src/lib.rs`:

- `mod_line`: `"[x] sodium"` when enabled, `"[ ] sodium"` when not.
- `mods_summary`: `"2 of 3 mods enabled"` for a list of `(name, enabled)` pairs.
