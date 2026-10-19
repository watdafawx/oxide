**Why now:** items are not all the same. A block is placed, a tool wears out, food gets eaten. An **enum**, `ItemKind`, says an item is *one of a few kinds*, and each kind carries its own data (tool durability, block hardness).

```rust
enum Mob {
    Bat,                                        // no data
    Zombie(u32),                                // one value: its hp
    Skeleton { arrows: u32, armoured: bool },   // named fields
}

impl Mob {
    fn threat(&self) -> u32 {
        match self {            // pick the code by variant
            Mob::Bat => 1,
            Mob::Zombie(hp) => *hp / 2,          // `hp` is a &u32, `*hp` is the number
            Mob::Skeleton { arrows, .. } => *arrows * 3,
        }
    }
}
```

- Create one with `Mob::Zombie(20)`.
- `match` is how you open an enum. The next quest covers it fully; here it works like a switch that hands you the data.
- Methods go in `impl Mob`, same as structs. `self` here is a borrowed `&Mob`, so `match self` binds references.
- Bound values are **references** (`hp` is a `&u32`): write `*hp` to get the number out.
- `_` means "I do not need this value".

## Your turn

- Open `src/lib.rs`. The `ItemKind` enum is given.
- Fill `uses` and `name`.
