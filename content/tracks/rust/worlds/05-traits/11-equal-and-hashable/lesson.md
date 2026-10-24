The paint tool remembers which grid cells a brush has touched, and how often. A `HashSet<Cell>` and a `HashMap<Cell, u32>` are the obvious tools, and Rust turns both down until `Cell` can answer two more questions.

**Try the obvious thing.** Collect the cells into a `HashSet` and the build stops:

```text
error[E0277]: the trait bound `Cell: Eq` is not satisfied
error[E0277]: the trait bound `Cell: Hash` is not satisfied
help: consider annotating `Cell` with `#[derive(Eq)]`
```

**Hash: where to look.** A `HashSet` turns each value into a number, its **hash**, to decide where to keep it and where to look for it later. **`Hash`** is the trait that does this, field by field.

**Eq: equal, for real.** `PartialEq` gives you `==`. **`Eq`** adds a promise and no new methods: every value is equal to itself. A set relies on it, so a key that went in can always be found again.

**Why "partial" exists.** An `f64` can hold NaN, "not a number", and `NaN == NaN` is false. So `f64` is `PartialEq` but not `Eq`, and a struct with an `f64` field cannot derive `Eq` or be a set key. Grid cells use `i32`, so they can.

```rust
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Tile {
    x: i32,
    y: i32,
}

let mut seen = HashSet::new();
seen.insert(Tile { x: 1, y: 2 });
seen.insert(Tile { x: 1, y: 2 });   // already there
// seen.len() is 1
```

**For the curious: keep them in step.** Two values that are `==` must have the same hash, or the set loses track of them. Deriving `PartialEq`, `Eq` and `Hash` together keeps that true. Write one of them by hand and you must write the others to match.

## Your turn

- In `distinct_cells`, collect the cells into a `HashSet` and count them. Save and read the errors.
- Add the two traits the errors name to `Cell`'s derive line.
- In `paint_counts`, count how many times each cell was painted, in a `HashMap`.
