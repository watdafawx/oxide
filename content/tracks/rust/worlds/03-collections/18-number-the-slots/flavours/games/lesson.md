The hotbar overlay labels its slots, `[1] sword`, `[2] bow`, the way the number keys pick them. And auto-refill wants to know *which* slots are empty, not just how many. Both need each item's position next to the item.

**Pair it with its position.** `.enumerate()` is an adaptor that turns each item into a pair: `(0, sword)`, `(1, bow)`, and so on. The position comes first, counts from 0, and is a `usize`.

**Take the pair apart.** A closure's parameter can be a pattern, like `let (a, b) = pair;` in the Foundations world. `|(i, item)|` names both halves of the pair at once. A half you do not need is `_`, as in a `match`: `|(i, _)|`.

```rust
let lines: Vec<String> = players
    .iter()
    .enumerate()
    .map(|(i, name)| format!("#{} {name}", i + 1))   // count from 1 for players
    .collect();
```

**Filter, then keep the position.** Put `filter` after `enumerate` and the positions stay attached to the items that pass. A `map(|(i, _)| i)` at the end keeps only the positions. Do `.copied()` before `enumerate`, so each pair holds a plain number.

**For the curious.** `a.iter().zip(b.iter())` pairs two lists item by item instead, stopping at the shorter one. `tiers.windows(2)` hands you each pair of neighbours, handy for "is the next tool an upgrade?".

## Your turn

- In `hotbar_labels`, enumerate the items and format each pair as `"[n] item"`, counting from 1.
- In `empty_slots`, enumerate the copied stack sizes, keep the zeros, and map each pair to its position.
