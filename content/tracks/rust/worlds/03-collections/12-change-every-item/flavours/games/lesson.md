A double-drops event doubles every drop in the loot table. A loop with a fresh `Vec` and a `push` works, but the line that matters, the rule, is buried in bookkeeping. An **iterator chain** says only the part that matters.

**One at a time.** `drops.iter()` makes an **iterator**: something that hands out the items of a list one at a time, as references, just like `for n in drops` did. On its own it does nothing yet; it waits to be asked.

**Change each one.** `.map(closure)` puts a rule between the list and you. Each item goes through the closure, and what comes out is the closure's answer. The closure gets a `&u32`; maths like `n * 2` follows the arrow for you.

**Gather them.** `.collect()` asks for every item and gathers the answers into a new collection. It can build several kinds, so it has to be told which. Here the function's return type, `Vec<u32>`, tells it.

```rust
let weights = vec![50, 30];
let heavy: Vec<u32> = weights.iter().map(|w| w + 10).collect();   // [60, 40]
```

**The list stays.** `iter` only lends the items, so the table is untouched and still usable. The chain built a new list next to it.

## Your turn

- In `doubled_drops`, write one chain: iter, map with `n * 2`, collect.
- In `drop_names`, map every `DropEntry` to a copy of its name: `d.item.clone()`.
