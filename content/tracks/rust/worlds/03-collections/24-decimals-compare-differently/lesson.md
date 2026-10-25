The shop's supplier sends prices with decimals, like 1.50 and 35.50, so they arrive as `f64`. Sorting them should be the same as sorting whole numbers. It is not, and the reason is one strange value.

**Try the obvious thing.** `prices.sort()` on a `Vec<f64>` does not build:

```text
error[E0277]: the trait bound `f64: Ord` is not satisfied
```

**The strange value.** An `f64` can hold **NaN**, "not a number", which is what `0.0 / 0.0` gives. NaN is not bigger than 1.0, not smaller, and not even equal to itself. With one value like that, decimals have no single natural order, so `f64` does not have the `Ord` trait that `sort` and `min_by_key` need.

**An order anyway.** `a.total_cmp(&b)` compares two `f64`s and always gives an `Ordering`: ordinary numbers in their usual order, NaN at the far end. Pass it to the closure methods: `sort_by` for sorting, and **`min_by`** / `max_by`, the siblings of `min_by_key` that take a compare closure instead of a key.

```rust
let mut weights = vec![2.5, 0.8, 1.2];
weights.sort_by(|a, b| a.total_cmp(b));                    // [0.8, 1.2, 2.5]
parcels.iter().max_by(|a, b| a.kg.total_cmp(&b.kg))        // the heaviest parcel
```

**For the curious.** Many languages let you sort decimals and quietly give a wrong order when a NaN is in the list. Rust makes you choose the rule, and `total_cmp` is the one that is always consistent.

## Your turn

- In `sorted_prices`, try `prices.sort()` and read the error. Then sort with `total_cmp`.
- In `cheapest`, pick the item with the lowest `price` using `min_by` and `total_cmp`.
