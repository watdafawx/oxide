Your `Countdown` only knows `next`. Yet it already has `map`, `filter`, `sum` and dozens more, because `Iterator` gives them to every type that writes `next`.

**Default methods, at scale.** `map`, `filter`, `sum` and `collect` are default methods of `Iterator`, the same idea as `shout` in the second quest. Each is written once in the standard library, on top of `next`.

**Write one, get dozens.** So `Countdown::new(5).filter(..)` works with no extra code. Your type plugs into every iterator tool you used in Collections, and into any written after it.

```rust
let evens: u32 = Up { cur: 0, max: 4 }
    .filter(|n| n % 2 == 0)
    .sum();                          // 2 + 4 = 6
```

**For the curious: why for works.** `for x in thing` first calls `thing.into_iter()` from the **`IntoIterator`** trait. Every iterator implements it by handing back itself, and `Vec`, arrays and `&Vec` implement it too, which is why all of them fit in a `for`.

## Your turn

- Write `evens`: the even numbers of a countdown from `start`, collected.
- Write `sum_of_squares`: the squares of a countdown from `start`, added up.
