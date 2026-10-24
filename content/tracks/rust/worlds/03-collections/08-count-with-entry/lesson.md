At closing time the till has a list of every sale: `pen, mug, pen, pen`. The owner wants a count per product. For each sale you add one to that product's count, and the first time a product shows up it has no count yet.

**One call for both cases.** `counts.entry(key)` is the slot in the map for that key, whether it is filled or not. `.or_insert(0)` fills an empty slot with 0. Either way you end up holding the count for that key, ready to change.

**Change it where it lives.** `or_insert` hands back a `&mut u32`: a mutable reference to the number inside the map, not a copy. Put `*` in front to follow it, the same `*` as before, and `+= 1` changes the number in the map itself.

```rust
let mut seen = HashMap::new();
for colour in ["red", "blue", "red"] {
    *seen.entry(colour.to_string()).or_insert(0) += 1;
}
// red: 2, blue: 1
```

**Why not get, then insert?** You could `get` the old count, add one and `insert` it back. That is two lookups and a `match` for the missing case. `entry` finds the slot once and handles both cases in one line.

**Why to_string.** The map owns its keys, so `entry` wants an owned `String`, not a borrowed `&str`. `product.to_string()` makes that copy; `get` can still look up with a plain `&str`.

## Your turn

- In `count_sales`, add one to each product's count inside the loop.
- In `add_delivery`, add `amount` to the product's count in `stock`. A new product starts at 0.
