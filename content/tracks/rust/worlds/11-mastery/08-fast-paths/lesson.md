**Why now:** the slow chunk indexer wastes work in four ordinary ways, and these four habits remove most of it.

Each habit is a measurable change, so this quest counts instead of timing.

- **`Vec::with_capacity(n)`**: when you know how many items are coming, reserve once instead of growing in steps.
- **No `clone()` in a loop**: hold a reference (`&T`) while you search, clone the winner once.
- **The `entry` API**: `map.entry(key).or_default()` does one lookup. `contains_key` + `get` + `insert` does three.
- **`&str` over `String`**: slicing and `split_whitespace` give you `&str` for free. `to_string()` and `collect::<Vec<String>>()` copy every word.

```rust
let mut counts: HashMap<&str, u32> = HashMap::new();
for block in palette.split_whitespace() {
    *counts.entry(block).or_default() += 1;  // one lookup per block name
}
```

```rust
let mut best: &String = &words[0];       // a reference: copying it is free
for w in &words {
    if w.len() > best.len() { best = w; }  // point at a new winner, no clone
}
let owned: String = best.clone();          // exactly one clone, at the very end
```

The test's `Counter` is a map of `String` to `u32`, so its `entry` wants an owned key: `counter.entry(word.to_string())`. That costs one copy but only one lookup. Filtering words without copying them: `text.split_whitespace().filter(|w| w.len() >= 3).count()`, since `filter` and `count` need no `Vec` at all.

`src/probe.rs` holds the meters (do not edit it): a `Counter` that counts every operation on it, and a `Name` whose `clone` is counted. `tests/fast.rs` counts allocations with the wrapper from the last quest.

## Your turn

In `src/lib.rs`, make each function do the same job with less work:

- `word_count`: one `Counter` operation per word.
- `longest`: at most one clone.
- `squares`: one allocation.
- `count_long_words`: zero allocations.
