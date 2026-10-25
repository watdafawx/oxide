Sale day: every price on the list drops by the same percent. A loop with a fresh `Vec` and a `push` works, but the line that matters, the price rule, is buried in bookkeeping. An **iterator chain** says only the part that matters.

**One at a time.** `prices.iter()` makes an **iterator**: something that hands out the items of a list one at a time, as references, just like `for p in prices` did. On its own it does nothing yet; it waits to be asked.

**Change each one.** `.map(closure)` puts a rule between the list and you. Each item goes through the closure, and what comes out is the closure's answer. The closure gets a `&u32`; maths like `p * 2` follows the arrow for you.

**Gather them.** `.collect()` asks for every item and gathers the answers into a new collection. It can build several kinds, so it has to be told which. Here the function's return type, `Vec<u32>`, tells it.

```rust
let prices = vec![150, 400];
let doubled: Vec<u32> = prices.iter().map(|p| p * 2).collect();   // [300, 800]
```

**The list stays.** `iter` only lends the items, so `prices` is untouched and still usable. The chain built a new list next to it.

## Your turn

- In `with_discount`, write one chain: iter, map with the discount rule, collect.
- In `sign_labels`, map every name to `n.to_uppercase()`.
