**Why now:** chunk positions are map keys, log lines and save-file entries. To log, compare, copy and print a `ChunkPos` you need the traits nearly every type has. Some you get for one line, some you write.

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]   // the compiler writes these impls
struct BlockPos { x: i32, y: i32 }

impl fmt::Display for BlockPos {              // you write this one: it is the user-facing text
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<i32> for BlockPos {                 // gives you .into() for free
    fn from(v: i32) -> Self { BlockPos { x: v, y: v } }
}

let a: BlockPos = 4.into();                   // From
let b = a.clone();                            // Clone
assert_eq!(a, b);                             // PartialEq
println!("{a} / {a:?}");                      // Display / Debug
let zero = BlockPos::default();               // 0 and 0
```

- **`derive`** works when every field already has the trait.
- **`Debug`** is for the console log, **`Display`** is for players.
- Implement **`From<X>`** and you get `.into()` automatically.

## Your turn

- Open `src/lib.rs`.
- Add the `derive` line above `struct ChunkPos`.
- Write the four helpers, `Display for ChunkPos` and `From<(i32, i32)> for ChunkPos`.
