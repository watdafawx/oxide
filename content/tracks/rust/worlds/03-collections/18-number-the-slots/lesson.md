The printed stock sheet numbers its lines, `1. pen`, `2. mug`, so staff can say "line 2 is wrong". And the reorder screen wants to know *which* shelves are low, not just how many. Both need each item's position next to the item.

**Pair it with its position.** `.enumerate()` is an adaptor that turns each item into a pair: `(0, pen)`, `(1, mug)`, and so on. The position comes first, counts from 0, and is a `usize`.

**Take the pair apart.** A closure's parameter can be a pattern, like `let (a, b) = pair;` in the Foundations world. `|(i, name)|` names both halves of the pair at once. A half you do not need is `_`, as in a `match`: `|(i, _)|`.

```rust
let lines: Vec<String> = staff
    .iter()
    .enumerate()
    .map(|(i, person)| format!("{}: {person}", i + 1))   // count from 1 for people
    .collect();
```

**Filter, then keep the position.** Put `filter` after `enumerate` and the positions stay attached to the items that pass. A `map(|(i, _)| i)` at the end keeps only the positions. Do `.copied()` before `enumerate`, so each pair holds a plain number.

**For the curious.** `a.iter().zip(b.iter())` pairs two lists item by item instead, stopping at the shorter one. `counts.windows(2)` hands you each pair of neighbours, handy for "did it go up?".

## Your turn

- In `numbered`, enumerate the names and format each pair as `"{n}. {name}"`, counting from 1.
- In `low_shelves`, enumerate the copied counts, keep the low ones, and map each pair to its position.
