**Why now:** `tp` copies a position around all the time, and nobody wants a move error just for using the same coordinates twice. A position is three small numbers. A player record holds a name, which lives on the heap. That is exactly the line between `Copy` and `Clone`.

- **`Copy`**: assignment and function calls silently duplicate the value. Numbers, `bool`, `char` and small plain structs work this way. No moves, ever.
- **`Clone`**: you call `.clone()` yourself. Used for anything that owns heap data (`String`, `Vec`), where copying costs something.

You rarely write these by hand: **derive** them.

```rust
#[derive(Debug, Clone, Copy)]
struct BlockPos { x: i32, y: i32, z: i32 }

#[derive(Debug, Clone)]
struct Player { name: String }

let pos = BlockPos { x: 1, y: 64, z: 2 };
let home = pos;                // copied, pos still works
let steve = Player { name: "Steve".to_string() };
let backup = steve.clone();    // explicit copy, steve still works
```

A struct can only be `Copy` if **every field** is `Copy`. A `String` field means Clone only.

## Your turn

- In `src/lib.rs`, edit the derive lines on `Player` and `BlockPos`.
- Fill in `duplicate_player` and `spawn_and_home`.
