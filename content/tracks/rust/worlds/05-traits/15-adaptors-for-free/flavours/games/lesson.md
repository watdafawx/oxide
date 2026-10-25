Your `ChunkRow` only knows `next`. Yet it already has `filter`, `map`, `sum` and dozens more, because `Iterator` gives them to every type that writes `next`.

**Default methods, at scale.** `map`, `filter`, `sum` and `collect` are default methods of `Iterator`, the same idea as `alert` in the second quest. Each is written once in the standard library, on top of `next`.

**Write one, get dozens.** So `ChunkRow::new(0, 8, 0).filter(..)` works with no extra code. Your type plugs into every iterator tool you used in Collections, and into any written after it.

```rust
let late: u32 = Ticks { now: 0, end: 4 }
    .filter(|t| t % 2 == 0)
    .sum();                          // 2 + 4 = 6
```

**For the curious: why for works.** `for x in thing` first calls `thing.into_iter()` from the **`IntoIterator`** trait. Every iterator implements it by handing back itself, and `Vec`, arrays and `&Vec` implement it too, which is why all of them fit in a `for`.

## Your turn

- Write `slime_chunks`: the chunks of the row whose `x` is even, collected.
- Write `total_distance_sq`: `x * x + z * z` of every chunk in the row, added up.
