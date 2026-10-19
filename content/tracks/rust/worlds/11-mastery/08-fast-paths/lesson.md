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

`src/probe.rs` holds the meters (do not edit it): a `Counter` that counts every operation on it, and a `Name` whose `clone` is counted. `tests/fast.rs` counts allocations with the wrapper from the last quest.

## Your turn

In `src/lib.rs`, make each function do the same job with less work:

- `word_count`: one `Counter` operation per word.
- `longest`: at most one clone.
- `squares`: one allocation.
- `count_long_words`: zero allocations.
