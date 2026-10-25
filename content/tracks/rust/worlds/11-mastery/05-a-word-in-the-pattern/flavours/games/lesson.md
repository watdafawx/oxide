The chunk indexer's block table reads best in the shape of the data: `block!("stone" => 1)` says "stone has id 1" at a glance, where `Block { name: "stone", id: 1 }` spells out field names on every row. The `=>` in the middle is not Rust syntax; your macro's pattern simply asks for it.

**Words that must be there.** Everything in a pattern that does not start with `$` is a **literal token**: the call must contain exactly that, at that spot. The comma in `($x:expr, $z:expr)` is one; so is the `=>` in `($name:expr => $id:expr)`. A call without it does not match the arm.

```rust
#[derive(Debug, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

#[macro_export]
macro_rules! chunk {
    ($x:expr ; $z:expr) => {
        ChunkPos { x: $x, z: $z }
    };
}

assert_eq!(chunk!(3; -4), ChunkPos { x: 3, z: -4 });
```

**Captures go anywhere.** The template is ordinary code with `$x` and `$z` pasted in. Here they land in a struct literal, the same `ChunkPos { x: .., z: .. }` you write by hand.

**Not every word fits.** After an `expr` capture only `=>`, `,` or `;` may come. `$name:expr at $id:expr` is refused, because Rust could not tell where an expression ends if any word were allowed after it. The starter's `=>` is one of the three.

## Your turn

- In `block!`, replace the stub template with a `Block` built from `$name` and `$id`.
