Two jobs for the price list. Prices go up 10% overnight, in the list itself. And the shop is closing: the last report needs only the names, and the products themselves will not be used again. The ownership world gave you three ways to hand over a value: lend it, lend it to change, or give it away. Walking a list has the same three.

**Look.** `products.iter()` lends each item, `&Product`, as every chain so far did. It is what `for p in &products` does. You can read the items; the list stays as it was.

**Change.** `products.iter_mut()` lends each item to change, `&mut Product`. It is what `for p in &mut products` does. Change a field, `p.price += 50`, or a number with `*n += 10`, and the change lands in the list itself.

**Take.** `products.into_iter()` gives the items away, `Product` itself, one by one. It is what `for p in products` does. The list is **moved** into the chain, so `products` cannot be used afterwards; in exchange you own each item and can take its parts out without copying.

```rust
for n in counts.iter_mut() {
    *n += 10;                    // every count in the list goes up
}
let labels: Vec<String> = signs.into_iter().map(|s| s.text).collect();   // signs is gone now
```

**Try taking with iter.** `products.iter().map(|p| p.name)` fails with `cannot move out of p.name which is behind a shared reference`: you were lent the product, so you cannot walk off with its name. Either `clone()` it, which copies the text, or own the list and use `into_iter()`.

## Your turn

- In `raise_prices`, walk the products with `iter_mut` and raise each `price`.
- In `into_names`, try `iter()` first and read the error. Then take the products with `into_iter()` and map each to `p.name`.
