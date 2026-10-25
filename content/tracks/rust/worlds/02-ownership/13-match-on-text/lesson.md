A `match` on text compares the value with literals, and literals are `&str`. Hand it an owned `String` and the two sides are different types, so Rust can't compare them.

**Try the obvious thing.** `match name { "admin" => 3, ... }` with `name: String` stops the build. The help line already names the fix:

```text
error[E0308]: mismatched types
help: consider converting the `String` to a `&str` using `.as_str()`
```

**Lend it as text.** `name.as_str()` lends the `String` as a `&str`, without copying a letter. Now each arm compares like with like.

```rust
fn shelf_for(kind: String) -> u32 {
    match kind.as_str() {
        "jam" => 1,
        "bread" => 2,
        _ => 9,
    }
}
```

**Nothing moves.** `as_str` only borrows, so `kind` is still owned by the function and is dropped at its `}` as usual.

**Why only here.** Most places that want a `&str` take `&name` and convert it for you. Patterns in a `match` are never converted, so that is where you write `.as_str()` yourself.

## Your turn

- In `rank_owned`, match on `name.as_str()`: `"admin"` gives 3, `"mod"` gives 2, anything else 1.
