No null in Rust. A value that might be missing is an **`Option<T>`**: `Some(value)` or `None`.

```rust
fn find_door(open: bool) -> Option<u32> {
    if open { Some(7) } else { None }
}

let door = find_door(true);
let n = door.unwrap_or(0);              // 7, or 0 when None
let next = door.map(|d| d + 1);         // Some(8); None stays None
```

- You can't use an `Option<u32>` as a `u32`. The compiler forces you to decide what `None` means.
- **`unwrap_or(default)`**: give me the value or this fallback.
- **`map(|x| ...)`**: transform the value inside, leave `None` alone. `|x| ...` is a tiny inline function (a closure).
- Avoid `.unwrap()`: it panics on `None`. Save it for tests and quick experiments.

## Your turn

- Open `src/lib.rs`.
- `half`: `Some` for even numbers, `None` for odd.
- `half_or_zero` and `half_plus_one`: build on `half` with `unwrap_or` and `map`.
