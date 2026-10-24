The loot screen has a "rare only" toggle. The answer is part of the loot table: only the drops with a rarity of 1 or more. `map` changes every item; this time you want to keep some and drop the rest.

**Let some through.** `.filter(closure)` asks the closure about each item. If it says `true`, the item goes on down the chain unchanged; if `false`, it is dropped. Like `map`, it is an adaptor: nothing happens until `collect` asks.

**Two arrows.** `filter` only looks at items, it does not take them, so it lends each one to the closure. After `.iter()` the items are already references, `&u32`, so the closure gets a reference to a reference: `&&u32`. Comparing it needs two stars, `**w`, which is hard to read.

**Copy first.** `.copied()` right after `.iter()` turns each `&u32` into a plain `u32`, as it did for `get`. Now the closure gets a `&u32` and one star does: `|w| *w > 10`. The `Vec` you collect holds plain numbers too.

```rust
let full: Vec<u32> = stacks.iter().copied().filter(|n| *n == 64).collect();
```

**Fields follow for you.** With a list of structs, `d.rarity` reaches the field through any number of arrows, so no stars are needed. Collecting gives a `Vec<&DropEntry>`: references to the entries in the original table, no copies.

## Your turn

- In `heavy_weights`, copy the numbers, keep those of at least `min`, collect.
- In `rare_drops`, filter the entries on `d.rarity` and collect the references.
