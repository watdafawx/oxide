The server reads each player's role from its config as a `String`, and the permission level depends on it. A `match` on text compares with literals, and literals are `&str`. Hand it an owned `String` and the two sides are different types, so Rust can't compare them.

**Try the obvious thing.** `match role { "owner" => 3, ... }` with `role: String` stops the build. The help line already names the fix:

```text
error[E0308]: mismatched types
help: consider converting the `String` to a `&str` using `.as_str()`
```

**Lend it as text.** `role.as_str()` lends the `String` as a `&str`, without copying a letter. Now each arm compares like with like.

```rust
fn gamemode_id(mode: String) -> u32 {
    match mode.as_str() {
        "survival" => 0,
        "creative" => 1,
        _ => 2,
    }
}
```

**Nothing moves.** `as_str` only borrows, so `mode` is still owned by the function and is dropped at its `}` as usual.

**Why only here.** Most places that want a `&str` take `&role` and convert it for you. Patterns in a `match` are never converted, so that is where you write `.as_str()` yourself.

## Your turn

- In `permission_level`, match on `role.as_str()`: `"owner"` gives 3, `"op"` gives 2, anything else 1.
