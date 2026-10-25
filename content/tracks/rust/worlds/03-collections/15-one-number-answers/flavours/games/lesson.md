The game rolls a loot drop against the table's total weight, and the balance screen asks how many entries are common. Both answers are one number, so the chain should end in a consumer that gives one number instead of a list.

**Add them up.** `.sum()` is a consumer: it pulls every item through the chain and adds them. As with `collect`, the return type tells it what kind of number to build, here `u32`. An empty table sums to 0. Use `map` first to pick the number out of each entry.

**Count what came through.** `.count()` pulls every item and only counts them. Put a `filter` before it and you count the items that passed. It always gives a `usize`, the type for lengths.

```rust
let rarity_total: u32 = table.iter().map(|d| d.rarity as u32).sum();
let legendary = table.iter().filter(|d| d.rarity == 3).count();       // a usize
```

**Replacing the loop.** These lines do what a loop with `let mut total` and a counter would do. The chain has no `mut` and no counter to get wrong, and it reads as the question.

**Big totals.** `sum` adds with `+`, so a `u32` total over about 4 billion panics in a test, just like `+` did in the Foundations world. Download counts get that big, which is why the boss keeps them in a `u64`.

## Your turn

- In `total_weight`, map each entry to its weight and sum.
- In `count_common`, keep the entries with a weight of at least `min_weight` and count them.
