A customer has 5 euros and asks what they can buy. The answer is part of the price list: only the products at or under 500 cents. `map` changes every item; this time you want to keep some and drop the rest.

**Let some through.** `.filter(closure)` asks the closure about each item. If it says `true`, the item goes on down the chain unchanged; if `false`, it is dropped. Like `map`, it is an adaptor: nothing happens until `collect` asks.

**Two arrows.** `filter` only looks at items, it does not take them, so it lends each one to the closure. After `.iter()` the items are already references, `&u32`, so the closure gets a reference to a reference: `&&u32`. Comparing it needs two stars, `**p`, which is hard to read.

**Copy first.** `.copied()` right after `.iter()` turns each `&u32` into a plain `u32`, as it did for `get`. Now the closure gets a `&u32` and one star does: `|p| *p > 100`. The `Vec` you collect holds plain numbers too.

```rust
let big: Vec<u32> = counts.iter().copied().filter(|n| *n > 10).collect();
```

**Fields follow for you.** With a list of structs, `p.price` reaches the field through any number of arrows, so no stars are needed. Collecting gives a `Vec<&Product>`: references to the products in the original list, no copies.

## Your turn

- In `cheap_prices`, copy the numbers, keep those at or under `max`, collect.
- In `affordable`, filter the products on `p.price` and collect the references.
