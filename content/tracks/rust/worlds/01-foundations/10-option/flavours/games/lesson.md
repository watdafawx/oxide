**Why now:** asking the recipe book for `"bedrock"` finds nothing. There is no null in Rust, so a lookup that might come back empty returns an **`Option<T>`**: `Some(value)` or `None`, and your calculator has to decide what "no recipe" means.

```rust
fn find_furnace_slot(busy: bool) -> Option<u32> {
    if busy { None } else { Some(7) }
}

let slot = find_furnace_slot(false);
let n = slot.unwrap_or(0);              // 7, or 0 when None
let next = slot.map(|s| s + 1);         // Some(8); None stays None
```

- You cannot use an `Option<u32>` as a `u32`. The compiler forces you to decide what `None` means.
- **`unwrap_or(default)`**: give me the value or this fallback.
- **`map(|x| ...)`**: transform the value inside, leave `None` alone. `|x| ...` is a tiny inline function (a closure).
- `match` also works on text. A `&str` matches string literals, and `_` catches the rest: `match item { "planks" => Some(4), _ => None }`.
- Avoid `.unwrap()`: it panics on `None`. Save it for tests and quick experiments.

## Your turn

- Open `src/lib.rs`.
- `recipe_output`: `Some(count)` for known recipes, `None` otherwise.
- `output_or_zero` and `double_batch`: build on `recipe_output` with `unwrap_or` and `map`.
