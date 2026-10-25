The owner's two favourite questions are "how many items are in the shop?" and "how many products are running low?". Both answers are one number, so the chain should end in a consumer that gives one number instead of a list.

**Add them up.** `.sum()` is a consumer: it pulls every item through the chain and adds them. As with `collect`, the return type tells it what kind of number to build, here `u32`. An empty list sums to 0.

**Count what came through.** `.count()` pulls every item and only counts them. Put a `filter` before it and you count the items that passed. It always gives a `usize`, the type for lengths.

```rust
let sold: u32 = sales.iter().sum();                                   // every unit sold
let big_orders = sales.iter().copied().filter(|n| *n >= 10).count();  // a usize
```

**Replacing the loop.** These two lines do what your `count_low` loop with `let mut low` did a few quests ago. The chain has no `mut` and no counter to get wrong, and it reads as the question.

**Big totals.** `sum` adds with `+`, so a `u32` total over about 4 billion panics in a test, just like `+` did in the Foundations world. When a total can grow that big, sum into a `u64`.

## Your turn

- In `total_units`, sum the counts.
- In `count_low`, keep the counts under `limit` and count them.
