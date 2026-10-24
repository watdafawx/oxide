The server keeps every chunk a player walks into loaded, and counts visits to pick which ones to keep warm. A `HashSet<ChunkPos>` and a `HashMap<ChunkPos, u32>` are the obvious tools, and Rust turns both down until `ChunkPos` can answer two more questions.

**Try the obvious thing.** Collect the positions into a `HashSet` and the build stops:

```text
error[E0277]: the trait bound `ChunkPos: Eq` is not satisfied
error[E0277]: the trait bound `ChunkPos: Hash` is not satisfied
help: consider annotating `ChunkPos` with `#[derive(Eq)]`
```

**Hash: where to look.** A `HashSet` turns each value into a number, its **hash**, to decide where to keep it and where to look for it later. **`Hash`** is the trait that does this, field by field.

**Eq: equal, for real.** `PartialEq` gives you `==`. **`Eq`** adds a promise and no new methods: every value is equal to itself. A set relies on it, so a key that went in can always be found again.

**Why "partial" exists.** An `f64` can hold NaN, "not a number", and `NaN == NaN` is false. So `f64` is `PartialEq` but not `Eq`, and a player's exact `f64` position cannot be a set key. Chunk positions are whole `i32`s, so they can.

```rust
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct BlockPos {
    x: i32,
    y: i32,
}

let mut lit = HashSet::new();
lit.insert(BlockPos { x: 1, y: 64 });
lit.insert(BlockPos { x: 1, y: 64 });   // already there
// lit.len() is 1
```

**For the curious: keep them in step.** Two values that are `==` must have the same hash, or the set loses track of them. Deriving `PartialEq`, `Eq` and `Hash` together keeps that true. Write one of them by hand and you must write the others to match.

## Your turn

- In `loaded_chunks`, collect the positions into a `HashSet` and count them. Save and read the errors.
- Add the two traits the errors name to `ChunkPos`'s derive line.
- In `visit_counts`, count how many times each chunk was visited, in a `HashMap`.
