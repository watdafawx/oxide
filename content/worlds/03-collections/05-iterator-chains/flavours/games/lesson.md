**Why now:** a `LootTable` is a list of `Drop`s, and the questions never stop: what can this zombie drop, which drops are rare, what is the total weight the game rolls against? Loops with a `mut` Vec and a counter work, but they bury the point. An **iterator chain** says what you want in a line: *take these, change them, keep some, give me a result*.

```rust
let weights = vec![50, 30, 10, 2];

let doubled: Vec<i32> = weights.iter().map(|w| w * 2).collect();
// [100, 60, 20, 4]    (Looting doubles every weight)

let heavy: Vec<i32> = weights.iter().copied().filter(|w| *w > 20).collect();
// [50, 30]

let total: i32 = weights.iter().sum();                        // 92
let light = weights.iter().filter(|w| **w < 20).count();      // 2
```

- **`.iter()`** starts a chain by borrowing each item (`&Drop`, `&i32`).
- **`.map(|x| ...)`** changes each item. **`.filter(|x| ...)`** keeps items where the closure is `true`.
- **`.copied()`** turns `&i32` into `i32`, handy before `filter`/`collect`.
- **`.collect()`** builds the result. Say what you want: `let v: Vec<i32> = ...` or `collect::<Vec<_>>()`.
- **`.sum()`** and **`.count()`** end the chain with a single number.

## Your turn

- Open `src/lib.rs`. `Drop` is given: an item, a rarity and a weight.
- Each of the four functions is one chain. No `for` loops, no `mut`.
