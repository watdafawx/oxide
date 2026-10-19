Moves are annoying for tiny values like a number. That's why some types skip them.

- **`Copy`**: assignment and function calls silently duplicate the value. Numbers, `bool`, `char` and small plain structs work this way. No moves, ever.
- **`Clone`**: you call `.clone()` yourself. Used for anything that owns heap data (`String`, `Vec`), where copying costs something.

You rarely write these by hand: **derive** them.

```rust
#[derive(Debug, Clone, Copy)]
struct Point { x: i32, y: i32 }

#[derive(Debug, Clone)]
struct Name { text: String }

let p = Point { x: 1, y: 2 };
let q = p;                 // copied, p still works
let n = Name { text: "a".to_string() };
let m = n.clone();         // explicit copy, n still works
```

A struct can only be `Copy` if **every field** is `Copy`. A `String` field means Clone only.

## Your turn

- In `src/lib.rs`, edit the derive lines on `Label` and `Point`.
- Fill in `two_labels` and `mirror`.
