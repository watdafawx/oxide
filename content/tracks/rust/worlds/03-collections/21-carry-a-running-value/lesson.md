A lamp costs 200 euros, and two discounts apply one after the other: 10% off, then 50% off what is left. That is not 60% off. The second discount works on the result of the first, so you need a value that is carried along the list and changed at every step.

**Carry it along.** `.fold(start, closure)` is a consumer that holds one value, often called the **accumulator** (`acc`), starting at `start`. For every item it calls the closure with the value so far and the item, and the closure's answer becomes the new value. At the end, `fold` gives back the last one.

**Read it as a loop.** `fold(0, |acc, n| acc + n)` is the loop `let mut acc = 0; for n in list { acc = acc + n; }`, without the `mut`. Any loop that updates one variable can be a fold; for a plain total, `sum()` already exists and reads better.

```rust
let deliveries = [5, 10, 3];
let stock = deliveries.iter().fold(12, |count, n| count + n);   // 12, 17, 27, 30 -> 30
```

**Order matters.** The closure sees the items in order, and each step works on the value the step before left behind. That is exactly what stacked discounts and a stock count after sales need.

**For the curious.** `sum`, `count` and `max` are all folds with the start and the step chosen for you. Reach for `fold` when no ready-made consumer fits.

## Your turn

- In `after_discounts`, fold over the percents, starting from `price`, taking each percent off what is left.
- In `stock_after_sales`, fold over the sales, starting from `start`, with `saturating_sub` so the count stops at 0.
